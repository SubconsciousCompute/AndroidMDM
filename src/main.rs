extern crate google_androidmanagement1 as androidmanagement1;
extern crate tokio;

use std::convert::Infallible;
use std::default::Default;
use std::env;
use std::sync::Arc;

use androidmanagement1::api::ListDevicesResponse;
use androidmanagement1::Result;
use androidmanagement1::{hyper, hyper_rustls, oauth2, AndroidManagement};
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, StatusCode};
use hyper_rustls::HttpsConnector;
use serde_json::to_string;

const CLOUD_PROJECT_ID: &'static str = "tonal-history-398005";
const ENTERPRISE_NAME: &'static str = "enterprises/LC03ulzkxb";

struct Android {
    hub: AndroidManagement<HttpsConnector<HttpConnector>>,
}

impl Android {
    #[must_use]
    async fn initiate() -> Self {
        let secret = oauth2::read_service_account_key("tonal-history-398005-4f72f8b989f3.json")
            .await
            .expect("tonal-history-398005-4f72f8b989f3.json");

        let secret = oauth2::ServiceAccountAuthenticator::builder(secret)
            .build()
            .await
            .expect("tonal-history-398005-4f72f8b989f3.json");

        Self {
            hub: AndroidManagement::new(
                hyper::Client::builder().build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .https_or_http()
                        .enable_http1()
                        .build(),
                ),
                secret,
            ),
        }
    }

    pub async fn list_devices(&self) -> Result<(Response<Body>, ListDevicesResponse)> {
        self.hub
            .enterprises()
            .devices_list(ENTERPRISE_NAME)
            .doit()
            .await
    }

    pub async fn enrollment_link(&self) -> String {
        let enrollment_token = self
            .hub
            .enterprises()
            .enrollment_tokens_create(Default::default(), ENTERPRISE_NAME)
            .doit()
            .await
            .unwrap()
            .1
            .value
            .unwrap();

        format!(
            "https://enterprise.google.com/android/enroll?et={}",
            enrollment_token
        )
    }
}

async fn handle_request(android: Arc<Android>, req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/devices") => match android.list_devices().await {
            Ok(response) => Ok(Response::new(Body::from(to_string(&response.1).unwrap()))),
            Err(_) => Ok(Response::new(Body::from("Error listing devices"))),
        },
        (&hyper::Method::GET, "/enroll") => {
            Ok(Response::new(Body::from(android.enrollment_link().await)))
        }
        _ => Ok(Response::new(Body::from("Not found"))),
    }
}

#[tokio::main]
async fn main() {
    let android = Android::initiate().await;
    let android = Arc::new(android);
    let service = make_service_fn(move |_| {
        let android = Arc::clone(&android);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(Arc::clone(&android), req)
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = hyper::Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
