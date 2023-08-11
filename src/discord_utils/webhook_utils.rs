use reqwest::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use crate::discord_utils::types::Webhook;

pub async fn post_webhook(client: &Client, webhook_url: String, webhook: &Webhook) -> Result<Response, String> {
    let resp: Response = client
        .post(&webhook_url)
        .json(&webhook)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();

    Ok(resp)
}