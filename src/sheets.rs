extern crate google_sheets4 as sheets4;

use google_sheets4::hyper::client::HttpConnector;
use std::default::Default;
use std::fs;

use google_sheets4::hyper_rustls::HttpsConnector;
use google_sheets4::oauth2::read_service_account_key;
use sheets4::api::ValueRange;
use sheets4::{hyper, hyper_rustls, oauth2, Sheets};
use sheets4::{Error, Result};

pub async fn append_to_spreadsheet(sheet_id: &str, range: &str) {
    let hub = get_service().await;

    let vr = serde_json::from_value(serde_json::json!({"values": [["2", "3", "4"]]})).unwrap();

    let result = hub
        .spreadsheets()
        .values_append(vr, sheet_id, range)
        .value_input_option("RAW")
        .include_values_in_response(true)
        .doit()
        .await;

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}

async fn get_service() -> Sheets<HttpsConnector<HttpConnector>> {
    let key = read_service_account_key("credentials.json").await.unwrap();
    let auth = oauth2::ServiceAccountAuthenticator::builder(key)
        .build()
        .await
        .unwrap();

    Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    )
}
