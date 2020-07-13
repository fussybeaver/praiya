use bytes::Bytes;
use std::borrow::Cow;
use std::future::Future;
use std::string;
use std::sync::Arc;
use std::time::Duration;

use futures_core::Stream;
use futures_util::future::TryFutureExt;
use futures_util::stream;
use futures_util::stream::StreamExt;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::request::Builder;
use hyper::client::HttpConnector;
use hyper::Uri as HyperUri;
use hyper::{Body, Method, Request, Response};
use hyper_rustls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::ser;

use crate::errors::Error::*;
use crate::errors::{self, Error};
use praiya_stubs::models::*;

type Client = hyper::Client<HttpsConnector<HttpConnector>>;
type TSMiddleware = Box<dyn Middleware + Send + Sync + 'static>;
type TSMiddlewareBuilder = Box<dyn MiddlewareBuilder<Inner = TSMiddleware> + Send + Sync + 'static>;

#[derive(Clone)]
pub struct Praiya {
    pub(crate) client: Arc<Client>,
    pub(crate) client_timeout: u64,
    pub(crate) token: Arc<String>,
    //pub(crate) middleware: Option<Arc<dyn Middleware + Send + Sync + 'static>>,
    pub(crate) middleware: Arc<Vec<TSMiddlewareBuilder>>,
}

#[derive(Debug)]
pub struct Uri<'a> {
    encoded: Cow<'a, str>,
}

impl<'a> Into<HyperUri> for Uri<'a> {
    fn into(self) -> HyperUri {
        self.encoded.as_ref().parse().unwrap()
    }
}

const PAGERDUTY_API_HOST: &str = "https://api.pagerduty.com";

/// Default timeout for all requests is 2 minutes.
const DEFAULT_TIMEOUT: u64 = 120;

impl Praiya {
    pub fn connect(
        token: &str,
        middleware: Option<Vec<TSMiddlewareBuilder>>,
    ) -> Result<Praiya, Error> {
        let mut config = rustls::ClientConfig::new();
        config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        config.ct_logs = Some(&ct_logs::LOGS);

        config.root_store = match rustls_native_certs::load_native_certs() {
            Ok(store) => store,
            Err((Some(store), err)) => {
                warn!("could not load all certificates: {}", err);
                store
            }
            Err((None, err)) => {
                warn!("cannot access native certificate store: {}", err);
                config.root_store
            }
        };

        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        let mut http_connector = HttpConnector::new();
        http_connector.enforce_http(false);

        let https_connector: HttpsConnector<HttpConnector> =
            HttpsConnector::from((http_connector, config));

        let client_builder = hyper::Client::builder();
        let client = Arc::new(client_builder.build(https_connector));

        Ok(Self {
            client,
            client_timeout: DEFAULT_TIMEOUT,
            token: Arc::new(token.to_string()),
            middleware: Arc::new(middleware.unwrap_or(vec![]).into_iter().collect()),
        })
    }

    pub(crate) fn build_request(
        &self,
        uri: Uri,
        builder: Builder,
        body: Body,
    ) -> Result<Request<Body>, Error> {
        let request_uri: hyper::Uri = uri.into();

        debug!("build request uri ({:?})", &request_uri);
        //let next_middleware = Arc::clone(self.middleware.as_ref().unwrap());
        //let next_middleware = self.middleware.as_ref().unwrap().build(

        Ok(builder
            .uri(request_uri)
            .header(CONTENT_TYPE, "application/json")
            .header(
                AUTHORIZATION,
                format!("Token token={}", self.token.as_ref()),
            )
            .body(body)?)
    }

    pub(crate) fn build_paginated_request(
        &self,
        path: &str,
        builder: Builder,
        query: Arc<dyn BaseOption + Send + Sync>,
        body: Body,
        pagination: PaginationQueryComponent,
    ) -> Result<Request<Body>, Error> {
        let uri = Praiya::parse_paginated_url(path, query, pagination)?;
        let request_uri: hyper::Uri = uri.into();

        debug!("build request uri ({:?})", &request_uri);

        Ok(builder
            .uri(request_uri)
            .header(CONTENT_TYPE, "application/json")
            .header(
                AUTHORIZATION,
                format!("Token token={}", self.token.as_ref()),
            )
            .body(body)?)
    }

    fn parse_url<'a>(path: &str, query: &str) -> Result<Uri<'a>, Error> {
        debug!("{}", path);
        let mut url = url::Url::parse(PAGERDUTY_API_HOST)?;
        url = url.join(path)?;
        url.set_query(Some(query));

        Ok(Uri {
            encoded: Cow::Owned(url.as_str().to_owned()),
        })
    }

    fn parse_paginated_url<'a>(
        path: &str,
        query: Arc<dyn BaseOption + Send + Sync>,
        pagination: PaginationQueryComponent,
    ) -> Result<Uri<'a>, Error> {
        let mut url = url::Url::parse(PAGERDUTY_API_HOST)?;
        url = url.join(path)?;

        url.set_query(Some(&query.build_paginated_query_string(pagination)));

        Ok(Uri {
            encoded: Cow::Owned(url.as_str().to_owned()),
        })
    }

    pub(crate) fn process_into_paginated_stream<
        'a,
        'b: 'a,
        T: DeserializeOwned,
        P: PaginatedResponse<Inner = Vec<T>> + DeserializeOwned + 'b,
    >(
        &'a self,
        base_req: BaseRequest,
        pagination: PaginationQueryComponent,
    ) -> impl Stream<Item = Result<T, Error>> + Unpin + 'a {
        let next_client = self.clone();
        let next_base_req = base_req.clone();
        Box::pin(
            self.process_request_with_middleware(base_req.build_request(self, pagination))
                .and_then(|(r, v)| Praiya::decode_response_with_middleware::<P, _>(r, v))
                .map_ok(|first| next_client.unfold(first, next_base_req))
                .try_flatten_stream(),
        )
    }

    fn unfold<P: PaginatedResponse<Inner = Vec<T>> + DeserializeOwned, T: DeserializeOwned>(
        self,
        first: P,
        base_req: BaseRequest,
    ) -> impl Stream<Item = Result<T, Error>> {
        let offset = first.get_offset();
        let limit = first.get_limit();
        let has_more = first.has_more();
        let iter = first.inner().into_iter();
        let cursor = PaginatedCursor {
            offset,
            has_more,
            limit,
        };
        Box::pin(stream::try_unfold(
            (self, base_req, cursor, iter),
            |(client, base_req, cursor, mut iter)| async {
                match iter.next() {
                    Some(val) => Ok(Some((val, (client, base_req, cursor, iter)))),
                    None if cursor.has_more => {
                        let res = client
                            .process_request_with_middleware(base_req.build_request(
                                &client,
                                PaginationQueryComponent {
                                    offset: cursor.offset + cursor.limit,
                                    limit: cursor.limit,
                                },
                            ))
                            .and_then(|(r, v)| {
                                Praiya::decode_response_with_middleware::<P, _>(r, v)
                            })
                            .await?;
                        let has_more = res.has_more();
                        let offset = res.get_offset();
                        let limit = res.get_limit();
                        let mut iter = res.inner().into_iter();
                        let cursor = PaginatedCursor {
                            offset,
                            has_more,
                            limit,
                        };
                        Ok(iter
                            .next()
                            .map(move |v| (v, (client, base_req, cursor, iter))))
                    }
                    None => Ok(None),
                }
            },
        ))
    }

    pub(crate) fn process_into_value<T, S: SingleResponse<Inner = T> + DeserializeOwned>(
        &self,
        req: Result<Request<Body>, Error>,
    ) -> impl Future<Output = Result<T, Error>> + '_
    where
        T: DeserializeOwned,
    {
        let fut = self.process_request_with_middleware(req);

        async move {
            let (response, vec_boxed) = fut.await?;
            Praiya::decode_response_with_middleware::<S, _>(response, vec_boxed)
                .await
                .map(|s| s.inner())
        }
    }

    fn process_request_with_middleware(
        &self,
        req: Result<Request<Body>, Error>,
    ) -> impl Future<Output = Result<(Response<Body>, Vec<TSMiddleware>), Error>> + '_ {
        // FIXME: refactor into more readable functions
        let res = self.middleware.as_ref().into_iter().fold(
            req.map(|r| (Ok(r), vec![])),
            |res, builder| {
                res.and_then(|(r, mut v)| match r {
                    Ok(rr) => {
                        let m = builder.build(&rr)?;
                        let r = m.request_handler(Ok(rr));
                        v.push(m);
                        Ok((r, v))
                    }
                    Err(e) => Err(e),
                })
            },
        );

        futures_util::future::ready(res.map(|r| (self, r)))
            .and_then(|(client, (r, v))| client.process_request(r).map_ok(|res| (res, v)))
    }

    fn process_request(
        &self,
        request: Result<Request<Body>, Error>,
    ) -> impl Future<Output = Result<Response<Body>, Error>> {
        let client = Arc::clone(&self.client);
        let timeout = self.client_timeout;

        async move {
            let request = request?;
            let response = Praiya::execute_request(client, request, timeout).await?;

            let status = response.status();
            match status {
                // Status code 200 - 299
                s if s.is_success() => Ok(response),

                s => {
                    let json: errors::Conflict = Praiya::decode_response(response).await?;
                    let message = json.error.as_ref().and_then(|e| e.message.as_ref());
                    let app_code = json.error.as_ref().and_then(|e| e.code).unwrap_or(-1);
                    Err(PraiyaResponseServerError {
                        message: message.unwrap_or(&String::new()).to_string(),
                        status_code: s.into(),
                        app_code,
                    })
                }
            }
        }
    }

    pub(crate) fn process_into_unit(
        &self,
        req: Result<Request<Body>, Error>,
    ) -> impl Future<Output = Result<(), Error>> + '_ {
        let fut = self.process_request_with_middleware(req);
        async move {
            let (response, vec_boxed) = fut.await?;

            // FIXME: refactor with decode_response
            let status_code = &response.status();
            let (parts, body) = response.into_parts();
            let headers = parts.headers;
            let res_bytes = Ok(hyper::body::to_bytes(body).await?);
            vec_boxed.into_iter().fold(res_bytes, |xs, r| {
                r.response_handler(status_code, &headers, xs)
            })?;

            Ok(())
        }
    }

    async fn execute_request(
        client: Arc<hyper::Client<HttpsConnector<HttpConnector>>>,
        req: Request<Body>,
        timeout: u64,
    ) -> Result<Response<Body>, Error> {
        let request = client.request(req);

        match tokio::time::timeout(Duration::from_secs(timeout), request).await {
            Ok(v) => v.map_err(Error::from),
            Err(_) => Err(RequestTimeoutError),
        }
    }

    async fn decode_into_string(response: Response<Body>) -> Result<String, Error> {
        let body = hyper::body::to_bytes(response.into_body()).await?;

        Ok(string::String::from_utf8_lossy(&body).to_string())
    }

    async fn decode_response_with_middleware<
        T: DeserializeOwned,
        I: IntoIterator<Item = TSMiddleware>,
    >(
        response: Response<Body>,
        middleware: I,
    ) -> Result<T, Error> {
        let status_code = &response.status();
        let (parts, body) = response.into_parts();
        let headers = parts.headers;
        let res_bytes = Ok(hyper::body::to_bytes(body).await?);
        let bytes = middleware.into_iter().fold(res_bytes, |xs, r| {
            r.response_handler(status_code, &headers, xs)
        })?;

        serde_json::from_str::<T>(&string::String::from_utf8_lossy(&bytes).to_string()).map_err(
            |e| {
                if e.is_data() {
                    JsonDataError {
                        message: e.to_string(),
                        column: e.column(),
                    }
                } else {
                    JsonDeserializeError { err: e }
                }
            },
        )
    }

    async fn decode_response<T: DeserializeOwned>(response: Response<Body>) -> Result<T, Error> {
        let contents = Praiya::decode_into_string(response).await?;

        debug!("Decoded into string: {}", &contents);
        serde_json::from_str::<T>(&contents).map_err(|e| {
            if e.is_data() {
                JsonDataError {
                    message: e.to_string(),
                    column: e.column(),
                }
            } else {
                JsonDeserializeError { err: e }
            }
        })
    }

    pub(crate) fn serialize_payload<S>(body: S) -> Result<Body, Error>
    where
        S: ser::Serialize,
    {
        Ok(serde_json::to_string(&body)
            .map(|content| content.into())
            .unwrap_or(Body::empty()))
    }

    pub fn services(&self) -> Services {
        Services {
            client: self.clone(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct BaseRequest {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) options: Arc<dyn BaseOption + Send + Sync>,
}

trait RequestBuilder {
    fn build_request(
        &self,
        client: &Praiya,
        offset: PaginationQueryComponent,
    ) -> Result<Request<Body>, Error>;
}

impl RequestBuilder for BaseRequest {
    fn build_request(
        &self,
        client: &Praiya,
        pagination: PaginationQueryComponent,
    ) -> Result<Request<Body>, Error> {
        client.build_paginated_request(
            &self.path,
            Builder::new().method(self.method.clone()),
            Arc::clone(&self.options),
            Body::empty(),
            pagination,
        )
    }
}

trait SubSystem {
    fn path(&self) -> &'static str;
}

pub(crate) trait PaginatedResponse {
    type Inner;

    fn get_offset(&self) -> usize;
    fn get_limit(&self) -> usize;
    fn inner(self) -> Self::Inner;
    fn has_more(&self) -> bool;
}

pub(crate) struct PaginatedCursor {
    pub(crate) offset: usize,
    pub(crate) has_more: bool,
    pub(crate) limit: usize,
}

impl PaginatedResponse for ServicesListResponse {
    type Inner = Vec<Service>;

    fn get_offset(&self) -> usize {
        self.offset
    }

    fn get_limit(&self) -> usize {
        self.limit
    }

    fn inner(self) -> Self::Inner {
        self.services
    }

    fn has_more(&self) -> bool {
        self.more
    }
}

pub(crate) trait SingleResponse {
    type Inner;

    fn inner(self) -> Self::Inner;
}

pub struct Services {
    client: Praiya,
}

pub trait Middleware {
    fn request_handler(
        &self,
        respond: Result<Request<Body>, Error>,
    ) -> Result<Request<Body>, Error>;

    fn response_handler(
        &self,
        status_code: &hyper::StatusCode,
        headers: &hyper::HeaderMap,
        respond: Result<Bytes, Error>,
    ) -> Result<Bytes, Error>;
}

pub trait MiddlewareBuilder {
    type Inner;

    fn build(&self, request: &Request<Body>) -> Result<Self::Inner, Error>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesListResponse {
    pub offset: usize,
    pub more: bool,
    pub limit: usize,
    pub total: Option<u64>,
    pub services: Vec<Service>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesGetResponse {
    pub service: Service,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesCreateResponse {
    pub service: Service,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesUpdateResponse {
    pub service: Service,
}

impl SingleResponse for ServicesGetResponse {
    type Inner = Service;

    fn inner(self) -> Self::Inner {
        self.service
    }
}

impl SingleResponse for ServicesCreateResponse {
    type Inner = Service;

    fn inner(self) -> Self::Inner {
        self.service
    }
}

impl SingleResponse for ServicesUpdateResponse {
    type Inner = Service;

    fn inner(self) -> Self::Inner {
        self.service
    }
}

impl SubSystem for Services {
    fn path(&self) -> &'static str {
        "/services"
    }
}

pub struct ServiceListOptionsBuilder<'a> {
    qs: form_urlencoded::Serializer<'a, String>,
}

pub struct ServiceListOptions {
    pub(crate) qs: String,
}

impl<'a> ServiceListOptionsBuilder<'a> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new()),
        }
    }

    pub fn include<T: Into<String>, I: IntoIterator<Item = T>>(&mut self, include: I) -> &mut Self {
        for item in include {
            let i: String = item.into();
            self.qs.append_pair("include[]", &i);
        }
        self
    }

    pub fn query<T: Into<String>>(&mut self, query: T) -> &mut Self {
        self.qs.append_pair("query", &query.into());

        self
    }

    pub fn sort_by<T: Into<String>>(&mut self, sort_by: T) -> &mut Self {
        self.qs.append_pair("sort_by", &sort_by.into());

        self
    }

    pub fn build(&mut self) -> ServiceListOptions {
        ServiceListOptions {
            qs: self.qs.finish(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct PaginationQueryComponent {
    pub offset: usize,
    pub limit: usize,
}

pub(crate) trait BaseOption {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String;
}

use url::form_urlencoded;

impl BaseOption for ServiceListOptions {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
        let mut query = form_urlencoded::Serializer::new(self.qs.clone());
        query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("limit", &pagination.limit.to_string());
        query.finish()
    }
}

pub struct ServiceGetOptions {
    pub(crate) qs: String,
}

pub struct ServiceGetOptionsBuilder<'a> {
    qs: form_urlencoded::Serializer<'a, String>,
}

impl<'a> ServiceGetOptionsBuilder<'a> {
    pub fn new() -> Self {
        Self {
            qs: form_urlencoded::Serializer::new(String::new()),
        }
    }

    pub fn include<T: Into<String>, I: IntoIterator<Item = T>>(&mut self, include: I) -> &mut Self {
        for item in include {
            let i: String = item.into();
            self.qs.append_pair("include[]", &i);
        }
        self
    }

    pub fn build(&mut self) -> ServiceGetOptions {
        ServiceGetOptions {
            qs: self.qs.finish(),
        }
    }
}

impl BaseOption for ServiceGetOptions {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
        let mut query = form_urlencoded::Serializer::new(self.qs.clone());
        query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("limit", &pagination.limit.to_string());
        query.finish()
    }
}

const DEFAULT_PAGERDUTY_API_LIMIT: usize = 25;

impl Services {
    pub fn list(
        &self,
        options: ServiceListOptions,
    ) -> impl Stream<Item = Result<Service, Error>> + '_ {
        let base_request = BaseRequest {
            method: Method::GET,
            options: Arc::new(options),
            path: String::from("/services"),
        };

        self.client
            .process_into_paginated_stream::<Service, ServicesListResponse>(
                base_request,
                PaginationQueryComponent {
                    offset: 0,
                    limit: DEFAULT_PAGERDUTY_API_LIMIT,
                },
            )
            .boxed()
    }

    pub async fn get(&self, id: &str, options: ServiceGetOptions) -> Result<Service, Error> {
        let uri = Praiya::parse_url(&format!("/services/{}", &id), &options.qs)?;

        let req = self
            .client
            .build_request(uri, Builder::new().method(Method::GET), Body::empty());

        self.client
            .process_into_value::<Service, ServicesGetResponse>(req)
            .await
    }

    pub async fn create(&self, service: Service) -> Result<Service, Error> {
        let uri = Praiya::parse_url(&"/services", "")?;

        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::POST),
            Praiya::serialize_payload(Some(service))?,
        );

        self.client
            .process_into_value::<Service, ServicesCreateResponse>(req)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let uri = Praiya::parse_url(&format!("/services/{}", &id), "")?;

        let req =
            self.client
                .build_request(uri, Builder::new().method(Method::DELETE), Body::empty());

        self.client.process_into_unit(req).await
    }

    pub async fn update(&self, service: Service) -> Result<Service, Error> {
        let uri = Praiya::parse_url(&"/services", "")?;

        let req = self.client.build_request(
            uri,
            Builder::new().method(Method::PUT),
            Praiya::serialize_payload(Some(service))?,
        );

        self.client
            .process_into_value::<Service, ServicesUpdateResponse>(req)
            .await
    }
}
