use std::error::Error;
use discord_utils::types::Webhook;
use crate::kosetto_api::kosetto_client;
use dotenvy::dotenv;
use reqwest::{Client};
use crate::discord_utils::types::{Author, Embed};
use crate::discord_utils::webhook_utils::post_webhook;

mod kosetto_api;
mod discord_utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv().expect("ERROR: Could not load .env file.");

    let webhook_url: String = std::env::var("WEBHOOK_URL")
        .expect("WEBHOOK_URL env has not been set.");

    let client = Client::new();

    let user_info = kosetto_client::get_user(&client, "test".to_string())
        .await;

    match user_info {
        Ok(user_info) => {

            let mut embed: Embed = Embed::new();

            embed.set_author(Author::new(user_info.users[0].twitter_name.clone(),
                                         user_info.users[0].twitter_pfp_url.clone()));

            embed.set_title("New User Sign Up".to_string());
            embed.set_description(format!("Wallet: {} \n Twitter Username: {}",
                                          user_info.users[0].address.clone(),
                                          user_info.users[0].twitter_username.clone()));

            let mut webhook = Webhook::new();
            webhook.set_embeds(vec!(embed));
            let resp = post_webhook(&client, webhook_url, &webhook).await;

            match resp {
                Ok(_) => {
                    println!("LOG: Posted webhook to discord.");
                }
                Err(_) => {
                    println!("ERROR: Could not post webhook to discord.");
                }
            }
        },
        Err(e) => {
            println!("ERROR: status code => {}", e);
        }
    }
    Ok(())
}
