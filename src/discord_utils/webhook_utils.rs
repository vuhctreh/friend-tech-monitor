use std::env;
use reqwest::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use eyre::{eyre, Result};
use crate::discord_utils::types::{Author, Embed, Webhook};
use crate::kosetto_api::types::User;

pub async fn post_webhook(client: &Client, webhook: &Webhook) -> Result<Response> {
    let webhook_url = env::var("WEBHOOK_URL")?;

    if webhook_url.is_empty() { return Err(eyre!("WEBHOOK_URL is not set.")) }

    let resp: Response = client
        .post(webhook_url)
        .json(&webhook)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    Ok(resp)
}

pub fn prepare_webhook(user: User) -> Webhook {
    log::info!("Preparing Discord embed.");

    let mut embed: Embed = Embed::new();

    embed.set_author(Author::new(&user.twitter_username, &user.twitter_pfp_url));

    embed.set_title("New User Sign Up".to_string());
    embed.set_description(format!("Wallet: {} \n Twitter Username: {}",
                                  &user.address,
                                  &user.twitter_name));

    let mut webhook: Webhook = Webhook::new();
    webhook.set_embeds(vec!(embed));

    webhook
}