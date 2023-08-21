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
// TODO: add sniper limit pricing and retries
// TODO: add take profit
// TODO: add inventory management

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    dotenv().expect("ERROR: Could not load .env file.");

    let delay = std::env::var("DELAY")
        .expect("ERROR: DELAY in .env was not set")
        .parse::<u64>().unwrap();

    let client: Client = Client::new();

    let config: WalletConfig = WalletConfig::new().await;

    loop {
        let res =  monitor(client.clone(), config.clone(), delay).await;

        match res {
            Ok(_) => {},
            Err(e) => log::error!("{:?}", e)
        }
    }
}