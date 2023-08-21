use std::error::Error;
use std::thread;
use std::time::Duration;
use dotenvy::dotenv;
use reqwest::{Client};
use simple_logger::SimpleLogger;
use crate::ethereum::config::WalletConfig;
use crate::monitor::monitor::monitor;
use text_io::read;
use crate::auth::sms::auth_impl::{init_sms_auth, verify_sms_auth};
use crate::io_utils::cli_utils::get_code_from_cli;

mod kosetto_api;
mod discord_utils;
mod io_utils;
mod ethereum;
mod monitor;
mod auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    dotenv().expect("ERROR: Could not load .env file.");

    let client = Client::new();

    let init_res = init_sms_auth(&client).await.unwrap();

    println!("SMS init status: {}", init_res.status());

    let code = get_code_from_cli();

    let auth_res = verify_sms_auth(&client, code).await.unwrap();

    println!("SMS auth status: {}", auth_res.status());
    println!("SMS auth message: {}", auth_res.text().await.unwrap());

    Ok(())
}


// TODO: make sniper and webhook parallel.
// TODO: add sniper retries
// TODO: add take profit
// TODO: add inventory management
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     SimpleLogger::new().init().unwrap();
//
//     dotenv().expect("ERROR: Could not load .env file.");
//
//     let delay = std::env::var("DELAY")
//         .expect("ERROR: DELAY in .env was not set")
//         .parse::<u64>().unwrap();
//
//     let client: Client = Client::new();
//
//     let config: WalletConfig = WalletConfig::new().await;
//
//     loop {
//         let res =  monitor(client.clone(), config.clone(), delay).await;
//
//         match res {
//             Ok(_) => {},
//             Err(e) => log::error!("{:?}", e)
//         }
//     }
// }