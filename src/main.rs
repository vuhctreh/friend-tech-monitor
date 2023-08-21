use std::error::Error;
use dotenvy::dotenv;
use reqwest::{Client};
use simple_logger::SimpleLogger;
use crate::ethereum::config::WalletConfig;
use crate::monitor::monitor::monitor;

mod kosetto_api;
mod discord_utils;
mod io_utils;
mod ethereum;
mod monitor;

// TODO: make sniper and webhook parallel.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    dotenv().expect("ERROR: Could not load .env file.");

    let webhook_url: String = std::env::var("WEBHOOK_URL")
        .expect("ERROR: WEBHOOK_URL env has not been set.");

    let delay = std::env::var("DELAY")
        .expect("ERROR: DELAY in .env was not set")
        .parse::<u64>().unwrap();

    let client: Client = Client::new();

    let config: WalletConfig = WalletConfig::new().await;

    monitor(client, config, &webhook_url, delay).await?;

    Ok(())
}

#[tokio::test]
async fn test_ethereum() -> Result<(), Box<dyn Error>> {
    dotenv().expect("ERROR: Could not load .env file.");

    ethereum::sniper::snipe(Default::default(), Default::default()).await.expect("TODO: panic message");

    Ok(())
}