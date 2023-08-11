use std::error::Error;
use discord_utils::types::Webhook;
use crate::kosetto_api::kosetto_client;
use dotenvy::dotenv;
use reqwest::{Client};
use crate::discord_utils::webhook_utils::post_webhook;

mod kosetto_api;
mod discord_utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv().expect("ERROR: Could not load .env file.");

    let webhook_url: String = std::env::var("WEBHOOK_URL")
        .expect("WEBHOOK_URL env has not been set.");

    let client = Client::new();

    let user_info = kosetto_client::get_user(&client, "lazekzeja".to_string())
        .await;

    match user_info {
        Ok(user_info) => {
            let webhook = Webhook::new(user_info.users[0].address.clone());
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
