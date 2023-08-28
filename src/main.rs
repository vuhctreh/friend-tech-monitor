//! A tool for monitoring and sniping "keys" (shares) for specific
//! users on friend.tech.
//!
//! Shares are represented by values in a smart contract on the Base
//! Ethereum L2 network.
//!
//! For more information on friend.tech please read the following article:
//!
//! [Gaining an edge on friend.tech with basic programming](https://fr4.hashnode.dev/gaining-an-edge-on-friendtech-with-basic-programming)
//!
//!
//! __Important Links:__
//! - `friend.tech PWA:` <https://www.friend.tech/>
//! - `friend.tech contract:` <https://basescan.org/address/0xcf205808ed36593aa40a44f10c7f7c2f67d4a4d4#readContract>
//! - `base:` <https://base.org/>

#![allow(unused)]
use std::error::Error;
use std::thread;
use dotenvy::dotenv;
use reqwest::{Client};
use simple_logger::SimpleLogger;
use crate::discord_utils::types::Webhook;
use crate::discord_utils::webhook_utils::{post_webhook, prepare_exception_embed};
use crate::ethereum::commons::WalletConfig;
use crate::monitor::monitor::monitor;

mod kosetto_api;
mod discord_utils;
mod io_utils;
mod ethereum;
mod monitor;
mod auth;
mod sniper;
mod monitor_v2;

// TODO: Monitor V2 (search blocks for friend.tech contract calls -> hit friend.tech user endpoint)
// TODO: Config  V2 (Move from .env to json, add modes for sniping and monitoring)
// TODO: tests
// TODO: add headless google auth
// TODO: add sniper retries
// TODO: add inventory management (separate service: TP, sell, view inv...)
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    dotenv().expect("ERROR: Could not load .env file.");

    loop {
        let res =  monitor(Client::new(), WalletConfig::new().await?).await;

        match res {
            Ok(_) => {},
            Err(e) => {
                log::error!("{:?}", e);
                let exception_hook: Webhook = prepare_exception_embed(e);
                post_webhook(&Client::new(), &exception_hook).await?;
                thread::sleep(std::time::Duration::from_secs(10));
                break;
            }
        }
    }

    Ok(())
}