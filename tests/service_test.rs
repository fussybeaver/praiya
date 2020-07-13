use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::io::Read;
use std::path;
use std::string;

use bytes::Bytes;
use futures_util::stream::StreamExt;
use futures_util::stream::TryStreamExt;
use http::uri::Parts;
use hyper::{Body, Method, Request};
use tokio::runtime::Runtime;

use praiya::errors::Error;
use praiya::models::*;
use praiya::praiya::{Middleware, MiddlewareBuilder};

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use slugify::slugify;

use mockito::mock;

pub struct MockFixtureMiddlewareBuilder {
    directory: path::PathBuf,
}

impl MockFixtureMiddlewareBuilder {
    pub fn new() -> MockFixtureMiddlewareBuilder {
        MockFixtureMiddlewareBuilder {
            directory: env::current_dir().unwrap().join("tests").join("fixtures"),
        }
    }
}

enum MaybeMock {
    Mock(mockito::Mock),
    File(path::PathBuf),
}

impl MiddlewareBuilder for MockFixtureMiddlewareBuilder {
    type Inner = Box<dyn Middleware + Send + Sync + 'static>;

    fn build(&self, req: &Request<Body>) -> Result<Self::Inner, Error> {
        let uri = req.uri();
        let method = req.method();
        let filename = generate_filename(self.directory.as_path(), uri, method);
        let maybe_mock = if path::Path::exists(filename.as_path()) {
            let mut file = std::fs::File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let cache: MockCacheObject = serde_json::from_str(&contents)?;
            let mut m = mock(
                req.method().as_str(),
                req.uri().path_and_query().map(|p| p.as_str()).unwrap_or(""),
            )
            .with_status(cache.status_code as usize);
            m = cache
                .headers
                .iter()
                .fold(m, |xs, (key, value)| xs.with_header(&key, &value));
            MaybeMock::Mock(m.with_body(cache.body).create())
        } else {
            MaybeMock::File(filename)
        };

        Ok(Box::new(MockFixtureMiddleware { maybe_mock }))
    }
}

fn generate_filename(dir: &path::Path, uri: &http::Uri, method: &Method) -> path::PathBuf {
    let parts: Parts = uri.clone().into();

    // FIXME: remove unwrap
    let filename = slugify!(&format!(
        "{}-{}",
        method,
        parts.path_and_query.unwrap().as_str()
    ));

    dir.join(filename).with_extension("json")
}

pub struct MockFixtureMiddleware {
    maybe_mock: MaybeMock,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MockCacheObject {
    status_code: u16,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

impl Middleware for MockFixtureMiddleware {
    fn request_handler(
        &self,
        request: Result<Request<Body>, Error>,
    ) -> Result<Request<Body>, Error> {
        match &self.maybe_mock {
            MaybeMock::Mock(_) => {
                let (mut parts, body) = request?.into_parts();
                let mockito_parts = mockito::server_url()
                    .parse::<http::uri::Uri>()
                    .unwrap()
                    .into_parts();

                let new_uri = http::uri::Uri::builder()
                    .scheme(mockito_parts.scheme.unwrap())
                    .authority(mockito_parts.authority.unwrap())
                    .path_and_query(parts.uri.path_and_query().unwrap().to_owned())
                    .build()?;

                debug!("mock new uri: {}", new_uri);

                parts.uri = new_uri;

                Ok(Request::from_parts(parts, body))
            }
            _ => request,
        }
    }

    fn response_handler(
        &self,
        status_code: &hyper::StatusCode,
        headers: &hyper::HeaderMap,
        respond: Result<Bytes, Error>,
    ) -> Result<Bytes, Error> {
        respond.and_then(|bytes| {
            match &self.maybe_mock {
                MaybeMock::File(filename) => {
                    let res: Result<HashMap<String, String>, Error> = headers
                        .into_iter()
                        .map(|(name, value)| Ok((name.to_string(), value.to_str()?.to_owned())))
                        .collect();

                    let cache = MockCacheObject {
                        status_code: status_code.as_u16(),
                        headers: res?,
                        body: string::String::from_utf8_lossy(&bytes).to_string(),
                    };
                    let cache = serde_json::to_string(&cache)?;
                    std::fs::write(filename.as_path(), cache)?;
                }
                _ => (),
            }
            Ok(bytes)
        })
    }
}

fn connect_to_pagerduty<F, Fut>(cb: F)
where
    F: FnOnce(praiya::Client) -> Fut,
    Fut: Future<Output = Result<(), Error>>,
{
    let mut rt = Runtime::new().unwrap();
    let token = std::env::var("PAGERDUTY_API_TOKEN").unwrap_or_else(|_| String::new());

    let client = praiya::Client::connect(
        &token,
        Some(vec![Box::new(MockFixtureMiddlewareBuilder::new())]),
    )
    .unwrap();

    rt.block_on(cb(client))
        .or_else(|e| {
            println!("{:?}", e);
            Err(e)
        })
        .unwrap();
}

async fn service_test(client: praiya::Client) -> Result<(), Error> {
    env_logger::init();

    // FIXME: get escalation policies first and pick one
    let service = Service {
        name: Some("Foo".to_string()),
        escalation_policy: Some(EscalationPolicyReference {
            id: Some("PWZCAZT".to_string()),
            _type: Some("escalation_policy_reference".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let service: Service = client.services().create(service).await?;
    assert_eq!("Foo", service.name.as_ref().unwrap());

    let sort_by = String::from("name:asc");
    let mut option = praiya::client::ServiceListOptionsBuilder::new();
    option.query("Foo").sort_by(&sort_by);

    let v = client
        .services()
        .list(option.build())
        .take(1)
        .try_collect::<Vec<_>>()
        .await?;

    assert!(!v.is_empty());

    let mut iter = v.into_iter();
    let first = iter.next().unwrap();
    let id = first.id.as_ref().unwrap();

    let mut option = praiya::client::ServiceGetOptionsBuilder::new();
    let include = vec!["escalation_policies", "teams"];
    option.include(include);

    let v: Service = client.services().get(id, option.build()).await?;

    assert_eq!("Foo", v.name.as_ref().unwrap());

    let err: Result<(), Error> = client.services().delete(id).await;

    assert!(err.is_ok());

    Ok(())
}

#[test]
fn integration_service_test() {
    connect_to_pagerduty(service_test);
}
