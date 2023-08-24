//! This module contains the implementations for SMS verification.

use reqwest::{Client, Response, StatusCode};
use reqwest::header::HeaderMap;
use reqwest::header::*;
use eyre::{eyre, Result};
use crate::auth::generate_header_map;
use crate::auth::sms::types::{SignTokenRequest, SignTokenResponse, SmsAuthRequest, SmsAuthResponse, SmsInitRequest};
use crate::io_utils::cli_utils::get_code_from_cli;

/// Calls privy /init endpoint to initiate SMS authentication.
/// This sends an SMS to the phone number specified in the request
/// containing a code.
pub async fn init_sms_auth(client: &Client) -> Result<Response> {
    const URL: &str = "https://auth.privy.io/api/v1/passwordless_sms/init";

    let sms_init_req = SmsInitRequest::new()?;

    let body = serde_json::to_string(&sms_init_req).unwrap();

    let headers: HeaderMap = generate_header_map();

    let res: Response = client.post(URL)
        .body(body)
        .headers(headers)
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        return Err(eyre!("Failed to init sms verification: {}", res.status()));
    }

    Ok(res)
}

/// Calls privy /authenticate endpoint to complete SMS authentication.
/// Requires the code sent via SMS.
///
/// An auth token is returned in the response.
pub async fn verify_sms_auth(client: &Client, code: String) -> Result<Response> {
    const URL: &str = "https://auth.privy.io/api/v1/passwordless_sms/authenticate";

    let sms_auth_req = SmsAuthRequest::new(code);

    let body = serde_json::to_string(&sms_auth_req).unwrap();

    let headers: HeaderMap = generate_header_map();

    let res: Response = client.post(URL)
        .body(body)
        .headers(headers)
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        return Err(eyre!("Failed to verify sms: {}", res.status()));
    }

    Ok(res)
}

/// Calls Kosetto signature endpoint to sign the auth token.
/// The phone number used in the previous steps must match
/// the address linked to that specific account.
pub async fn sign_auth_token(client: &Client, address: &str, token: String) -> Result<Response> {
    const URL: &str = "https://prod-api.kosetto.com/signature";

    let sign_auth_req = SignTokenRequest::new(address);

    let body = serde_json::to_string(&sign_auth_req)?;

    let mut headers: HeaderMap = generate_header_map();

    headers.insert(AUTHORIZATION, token.parse()?);

    let res: Response = client.post(URL)
        .body(body)
        .headers(headers)
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        return Err(eyre!("Failed to sign auth token: {}", res.status()));
    }
    
    Ok(res)
}


/// Combines all authentication steps into a single call.
pub async fn generate_auth_token(client: &Client) -> Result<String> {
    // TODO: search for wallet in LinkedAccounts rather than using fixed index
    init_sms_auth(client).await?;

    let code: String = get_code_from_cli();

    let verify_res = verify_sms_auth(client, code).await?;

    let verify_res_body: SmsAuthResponse = serde_json::from_str(&*verify_res.text().await?)?;

    let sign_address = &verify_res_body.user.linked_accounts[1].address.clone().unwrap();

    let signature_response = sign_auth_token(&client, sign_address, verify_res_body.token).await?;

    let full_resp: SignTokenResponse = serde_json::from_str(signature_response.text().await?.as_str())?;

    Ok(full_resp.token)
}