extern crate google_sheets4 as sheets4;

use std::default::Default;
use std::fs;

use google_sheets4::hyper_rustls::HttpsConnector;
use google_sheets4::oauth2::read_service_account_key;
use sheets4::{Error, Result};
use sheets4::{hyper, hyper_rustls, oauth2, Sheets};
use sheets4::api::ValueRange;

fn read_credentials() -> oauth2::ApplicationSecret {
    let contents = fs::read_to_string("credentials.json").unwrap();
    let secrets: oauth2::ApplicationSecret = serde_json::from_str(&contents).unwrap();
    return secrets;
}

pub async fn get_service() {
    let secret: oauth2::ApplicationSecret = read_credentials();
    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
    let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    let mut req = ValueRange::default();

    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
        .value_input_option("amet.")
        .response_value_render_option("duo")
        .response_date_time_render_option("ipsum")
        .insert_data_option("gubergren")
        .include_values_in_response(true)
        .doit().await;

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

pub async fn append_to_spreadsheet(sheet_id: &str, range: &str, values: Vec<Vec<String>>) {
    let key = read_service_account_key("credentials.json").await.unwrap();
    let auth = oauth2::ServiceAccountAuthenticator::builder(key).build().await.unwrap();

    let mut hub = Sheets::new(
        hyper::Client::builder()
            .build(hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build()), auth);

    let mut req = ValueRange::default();

    let result = hub.spreadsheets().values_append(req, sheet_id, range)
        .value_input_option("amet.")
        .response_value_render_option("duo")
        .response_date_time_render_option("ipsum")
        .insert_data_option("gubergren")
        .include_values_in_response(true)
        .doit().await;

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
