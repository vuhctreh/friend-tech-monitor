//! Implementation for Discord webhook utilities.

use std::env;
use std::sync::Arc;
use reqwest::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use eyre::{eyre, Report, Result};
use crate::discord_utils::types::{Author, Embed, Webhook};
use crate::kosetto_api::types::{User};

/// Posts a webhook to Discord.
pub async fn post_webhook(client: Arc<Client>, webhook: &Webhook) -> Result<Response> {
    log::info!("Posting webhook to Discord...");

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

/// Creates an embed to be posted upon the monitor detecting a user signing up.
pub fn prepare_user_signup_embed<T: User>(user: T) -> Webhook {
    log::info!("Preparing user signup Discord embed.");

    let mut embed: Embed = Embed::new();

    embed.set_author(Author::new(&user.get_username(), &user.get_pfp_url()));

    embed.set_title("New User Sign Up".to_string());
    embed.set_description(format!("Wallet: {} \n Twitter Username: {}",
                                  &user.get_address(),
                                  &user.get_name()));

    let mut webhook: Webhook = Webhook::new();
    webhook.set_embeds(vec!(embed));

    webhook
}

/// Creates an embed to be posted if an unhandled exception occurs and the program
/// needs to panic.
pub fn prepare_exception_embed(error: Report) -> Webhook {
    log::info!("Preparing exception Discord embed.");

    let mut embed: Embed = Embed::new();

    embed.set_title(format!("Bot shutting down due to the following:"));
    embed.set_description(format!("Exception: {}", &error.to_string()));

    let mut webhook: Webhook = Webhook::new();
    webhook.set_embeds(vec!(embed));

    webhook
}