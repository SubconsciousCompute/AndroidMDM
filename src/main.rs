extern crate google_androidmanagement1 as androidmanagement1;

use std::default::Default;
use androidmanagement1::{hyper, hyper_rustls, oauth2, AndroidManagement};

const CLOUD_PROJECT_ID: &'static str = "tonal-history-398005";
const ENTERPRISE_NAME: &'static str = "enterprises/LC03ulzkxb";

#[tokio::main]
async fn main() {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    let secret = oauth2::read_service_account_key("tonal-history-398005-4f72f8b989f3.json")
        .await
        .expect("tonal-history-398005-4f72f8b989f3.json");

    let secret = oauth2::ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .expect("tonal-history-398005-4f72f8b989f3.json");

    let mut hub = AndroidManagement::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        secret,
    );

    let devices = hub.enterprises().devices_list(ENTERPRISE_NAME).doit().await;

    // Show current enrolled devices
    println!("{:#?}", devices);
}
