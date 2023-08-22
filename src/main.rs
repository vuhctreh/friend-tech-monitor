use std::error::Error;
use std::thread;
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
mod auth;

// TODO: webhooks for panic
// TODO: add headless google auth
// TODO: make sniper and webhook parallel.
// TODO: add sniper retries
// TODO: add take profit
// TODO: add inventory management
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    dotenv().expect("ERROR: Could not load .env file.");

    loop {
        let res =  monitor(Client::new(), WalletConfig::new().await?, std::env::var("DELAY")?
            .parse::<u64>()?).await;

        match res {
            Ok(_) => {},
            Err(e) => {
                log::error!("{:?}", e);
                thread::sleep(std::time::Duration::from_secs(10));
                break;
            }
        }
    }

    Ok(())
}