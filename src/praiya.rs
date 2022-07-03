use bytes::Bytes;
use http::{HeaderMap, HeaderValue};
use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::string;
use std::sync::Arc;
use std::time::Duration;

use futures_core::Stream;
use futures_util::future::TryFutureExt;
use futures_util::stream;
use futures_util::stream::StreamExt;
use http::header::{HeaderName, ACCEPT, AUTHORIZATION, CONTENT_TYPE, FROM};
use http::request::Builder;
use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use hyper::Uri as HyperUri;
use hyper::{Body, Method, Request, Response};
use hyper_rustls::HttpsConnector;
use log::{debug, warn};
use serde::de::DeserializeOwned;
use serde::ser;

use crate::endpoints::incidents_macro;
use crate::errors::Error::*;
use crate::errors::{self, Error};
use crate::models::*;

type Client = hyper::Client<HttpsConnector<HttpConnector>>;

#[derive(Clone)]
pub struct Praiya {
    pub(crate) client: Arc<Client>,
    pub(crate) client_timeout: u64,
    pub(crate) token: Arc<String>,
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

/// Protection against malicious actor payload length
const MAX_ALLOWED_RESPONSE_SIZE: u64 = 32768;

pub enum PraiyaCustomHeaders {
    None,
    EarlyAccess,
}

impl Praiya {
    pub fn connect(token: &str) -> Result<Praiya, Error> {
        let https_connector: HttpsConnector<HttpConnector> =
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build();

        let client_builder = hyper::Client::builder();
        let client = Arc::new(client_builder.build(https_connector));

        Ok(Self {
            client,
            client_timeout: DEFAULT_TIMEOUT,
            token: Arc::new(token.to_string()),
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

        Ok(builder
            .uri(request_uri)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .header(
                AUTHORIZATION,
                format!("Token token={}", self.token.as_ref()),
            )
            .body(body)?)
    }

    pub(crate) fn build_paginated_request(
        &self,
        host: &str,
        path: &str,
        builder: Builder,
        query: Arc<dyn BaseOption + Send + Sync>,
        body: Body,
        pagination: PaginationQueryComponent,
    ) -> Result<Request<Body>, Error> {
        let uri = Praiya::parse_paginated_url(host, path, query, pagination)?;
        let request_uri: hyper::Uri = uri.into();

        debug!("build request uri ({:?})", &request_uri);

        Ok(builder
            .uri(request_uri)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .header(
                AUTHORIZATION,
                format!("Token token={}", self.token.as_ref()),
            )
            .body(body)?)
    }

    pub(crate) fn parse_url<'a>(host: &str, path: &str) -> Result<Uri<'a>, Error> {
        debug!("parse url path: {}", path);
        let mut url = url::Url::parse(host)?;
        url = url.join(path)?;

        Ok(Uri {
            encoded: Cow::Owned(url.as_str().to_owned()),
        })
    }

    fn parse_paginated_url<'a>(
        host: &str,
        path: &str,
        query: Arc<dyn BaseOption + Send + Sync>,
        pagination: PaginationQueryComponent,
    ) -> Result<Uri<'a>, Error> {
        let mut url = url::Url::parse(host)?;
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
            self.process_request(base_req.build_request(self, pagination))
                .and_then(Praiya::decode_response)
                .map_ok(|first: P| next_client.unfold(first, next_base_req))
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
                        let res: P = client
                            .process_request(base_req.build_request(
                                &client,
                                PaginationQueryComponent {
                                    offset: cursor.offset + cursor.limit,
                                    limit: cursor.limit,
                                },
                            ))
                            .and_then(Praiya::decode_response)
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
        let fut = self.process_request(req);

        async move {
            let response = fut.await?;
            Praiya::decode_response(response)
                .await
                .map(|s: S| s.inner())
        }
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
        let fut = self.process_request(req);
        async move {
            fut.await?;

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

    async fn decode_response<T: DeserializeOwned>(response: Response<Body>) -> Result<T, Error> {
        // Protect against malicious response
        let response_content_length = match response.body().size_hint().upper() {
            Some(v) => v,
            None => MAX_ALLOWED_RESPONSE_SIZE + 1,
        };

        if response_content_length < MAX_ALLOWED_RESPONSE_SIZE {
            let bytes = hyper::body::to_bytes(response.into_body()).await?;

            debug!(
                "Decoded into string: {}",
                &string::String::from_utf8_lossy(&bytes)
            );

            serde_json::from_slice::<T>(&bytes).map_err(|e| {
                if e.is_data() {
                    JsonDataError {
                        message: e.to_string(),
                        column: e.column(),
                    }
                } else {
                    JsonDeserializeError { err: e }
                }
            })
        } else {
            Err(OversizedPayloadError {
                len: response_content_length,
            })
        }
    }

    pub(crate) fn serialize_payload<S>(body: S) -> Result<Body, Error>
    where
        S: ser::Serialize,
    {
        Ok(serde_json::to_string(&body)
            .map(|content| content.into())?)
    }

    pub fn incidents(&self) -> incidents_macro::IncidentsClient {
        incidents_macro::IncidentsClient {
            api_endpoint: std::env::var("PAGERDUTY_API_ENDPOINT")
                .unwrap_or_else(|_| String::from(incidents_macro::API_ENDPOINT)),
            client: self.clone(),
        }
    }

    pub async fn post_request<
        B: serde::Serialize,
        R: DeserializeOwned,
        I: SingleResponse<Inner = R> + DeserializeOwned,
    >(
        &self,
        url: Uri<'_>,
        body: B,
        headers: PraiyaCustomHeaders,
    ) -> Result<R, Error> {
        let mut builder = http::request::Builder::new();
        builder = builder.header(FROM, "foobar@example.com");
        if let PraiyaCustomHeaders::EarlyAccess = headers {
            builder = builder.header("x-early-access", "true");
        }

        let req = self.build_request(
            url,
            builder.method(http::Method::POST),
            Praiya::serialize_payload(body)?,
        );

        self.process_into_value::<R, I>(req).await
    }

    pub async fn put_request<
        B: serde::Serialize,
        R: DeserializeOwned,
        I: SingleResponse<Inner = R> + DeserializeOwned,
    >(
        &self,
        url: Uri<'_>,
        body: B,
        headers: PraiyaCustomHeaders,
    ) -> Result<R, Error> {
        let mut builder = http::request::Builder::new();
        builder = builder.header(FROM, "foobar@example.com");
        if let PraiyaCustomHeaders::EarlyAccess = headers {
            builder = builder.header("x-early-access", "true");
        }

        let req = self.build_request(
            url,
            builder.method(http::Method::PUT),
            Praiya::serialize_payload(body)?,
        );

        self.process_into_value::<R, I>(req).await
    }

    pub fn list_request<
        R: DeserializeOwned + Sync + Send + 'static,
        B: BaseOption + 'static,
        I: PaginatedResponse<Inner = Vec<R>> + DeserializeOwned + 'static,
    >(
        &self,
        host: &str,
        path: &str,
        query_params: B,
        headers: PraiyaCustomHeaders,
    ) -> impl Stream<Item = Result<R, Error>> + '_ {
        let mut header_map = HashMap::new();
        if let PraiyaCustomHeaders::EarlyAccess = headers {
            header_map.insert(String::from("x-early-access"), String::from("true"));
        }
        let base_request = BaseRequest {
            host: String::from(host),
            method: Method::GET,
            options: Arc::new(query_params),
            path: String::from(path),
            headers: header_map,
        };

        debug!("host: {}", host);

        self.process_into_paginated_stream::<R, I>(
            base_request,
            PaginationQueryComponent {
                offset: 0,
                limit: DEFAULT_PAGERDUTY_API_LIMIT,
            },
        )
        .boxed()
    }

    pub async fn get_request<
        R: DeserializeOwned,
        I: SingleResponse<Inner = R> + DeserializeOwned,
    >(
        &self,
        url: Uri<'_>,
        headers: PraiyaCustomHeaders,
    ) -> Result<R, Error> {
        let mut builder = http::request::Builder::new();
        builder = builder.header(FROM, "foobar@example.com");
        if let PraiyaCustomHeaders::EarlyAccess = headers {
            builder = builder.header("x-early-access", "true");
        }

        let req = self.build_request(url, builder.method(Method::GET), Body::empty());

        self.process_into_value::<R, I>(req).await
    }
}

#[derive(Clone)]
pub(crate) struct BaseRequest {
    pub(crate) host: String,
    pub(crate) method: Method,
    pub(crate) options: Arc<dyn BaseOption + Send + Sync>,
    pub(crate) path: String,
    pub(crate) headers: HashMap<String, String>,
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
        let mut builder = Builder::new().method(self.method.clone());
        for (key, value) in self.headers.iter() {
            builder = builder.header(key, value);
        }
        client.build_paginated_request(
            &self.host,
            &self.path,
            builder,
            Arc::clone(&self.options),
            Body::empty(),
            pagination,
        )
    }
}

pub(crate) trait SubSystem {
    fn path(&self) -> &'static str;
}

pub trait PaginatedResponse {
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

pub trait SingleResponse {
    type Inner;

    fn inner(self) -> Self::Inner;
}

#[derive(Debug, PartialEq, Serialize)]
pub struct PaginationQueryComponent {
    pub offset: usize,
    pub limit: usize,
}

pub(crate) trait ParamsBuilder<B: BaseOption> {
    fn build(&mut self) -> B;
}

pub trait BaseOption: Send + Sync {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String;
}

use url::form_urlencoded;

pub const DEFAULT_PAGERDUTY_API_LIMIT: usize = 25;

#[derive(Default, Serialize)]
pub(crate) struct NoopParams {}

impl BaseOption for NoopParams {
    fn build_paginated_query_string(&self, pagination: PaginationQueryComponent) -> String {
        let mut query = url::form_urlencoded::Serializer::new(String::new());
        query.append_pair("offset", &pagination.offset.to_string());
        query.append_pair("limit", &pagination.limit.to_string());
        query.finish()
    }
}
