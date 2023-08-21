use reqwest::{Client, Error, Response};
use reqwest::header::HeaderMap;
use reqwest::header::*;
use crate::auth::generate_header_map;
use crate::auth::sms::types::{SmsAuthRequest, SmsInitRequest};

// TODO: make generic
pub async fn init_sms_auth(client: &Client) -> Result<Response, Error> {
    const URL: &str = "https://auth.privy.io/api/v1/passwordless_sms/init";

    let sms_init_req = SmsInitRequest::new();

    let body = serde_json::to_string(&sms_init_req).unwrap();

    let headers: HeaderMap = generate_header_map();

    let res: Response = client.post(URL)
        .body(body)
        .headers(headers)
        .send()
        .await?;

    Ok(res)
}

pub async fn verify_sms_auth(client: &Client, code: String) -> Result<Response, Error> {
    const URL: &str = "https://auth.privy.io/api/v1/passwordless_sms/authenticate";

    let sms_auth_req = SmsAuthRequest::new(code);

    let body = serde_json::to_string(&sms_auth_req).unwrap();

    let headers: HeaderMap = generate_header_map();

    let res: Response = client.post(URL)
        .body(body)
        .headers(headers)
        .send()
        .await?;

    Ok(res)
}