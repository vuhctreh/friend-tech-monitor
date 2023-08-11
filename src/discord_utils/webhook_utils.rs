use reqwest::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use crate::discord_utils::types::{Author, Embed, Webhook};

pub async fn post_webhook(client: &Client, webhook_url: &String, webhook: &Webhook) -> Result<Response, String> {
    let resp: Response = client
        .post(webhook_url)
        .json(&webhook)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();

    Ok(resp)
}

pub fn prepare_webhook(twitter_username: String, twitter_pfp_url: String, address: String, twitter_name: String) -> Webhook {
    let mut embed: Embed = Embed::new();

    embed.set_author(Author::new(twitter_username, twitter_pfp_url));

    embed.set_title("New User Sign Up".to_string());
    embed.set_description(format!("Wallet: {} \n Twitter Username: {}",
                                  address,
                                  twitter_name));

    let mut webhook: Webhook = Webhook::new();
    webhook.set_embeds(vec!(embed));

    webhook
}