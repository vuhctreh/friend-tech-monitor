use reqwest::Client;
use crate::kosetto_api::types::KosettoResponse;
use reqwest::StatusCode;

pub async fn get_user(client: &Client, user: String) -> Result<KosettoResponse, StatusCode> {

    let url: String = format!("{}{}", std::env::var("KOSETTO_URL").unwrap(), user);

    let resp = client.get(url)
        .send()
        .await
        .expect("ERROR: Failed to get user from Kosetto API.");

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<KosettoResponse>().await.expect("ERROR: Failed to parse Kosetto response.")),
        StatusCode::NOT_FOUND => Err(StatusCode::NOT_FOUND),
        _ => Err(resp.status()),
    }
}