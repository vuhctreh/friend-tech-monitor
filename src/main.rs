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
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use dotenvy::dotenv;
use ethers::contract::Abigen;
use ethers::middleware::Middleware;
use ethers::types::{BlockNumber, U64};
use lapin::Channel;
use log::{info, log};
use reqwest::{Client};
use simple_logger::SimpleLogger;
use crate::discord_utils::types::Webhook;
use crate::discord_utils::webhook_utils::{post_webhook, prepare_exception_embed};
use crate::ethereum::commons::ApplicationCommons;
use crate::monitor_v2::monitor_v2::monitor_v2;
use crate::rabbitmq::init_broker::rabbit_init;

mod kosetto_api;
mod discord_utils;
mod io_utils;
mod ethereum;
mod auth;
mod sniper;
mod monitor_v2;
mod rabbitmq;

// TODO: Config V2 (Move from .env to json, add modes for sniping and monitoring)
// TODO: add inventory management (separate service: TP, sell, view inv...)
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    dotenv().expect("ERROR: Could not load .env file.");

    let channel: Arc<Channel> = Arc::new(rabbit_init().await?);

    let commons: ApplicationCommons = ApplicationCommons::new()?;

    let mut block_number: U64 = commons.provider.get_block_number().await?;

    info!("Initial block number: {}", &block_number);

    loop {
        info!("Current block number: {}", &block_number);

        let res = monitor_v2(&commons, BlockNumber::from(block_number), channel.clone()).await;

        match res {
            Ok(1) => {
                block_number = block_number + 1;
            }
            Err(e) => {
                log::error!("{:?}", e);
                let exception_hook: Webhook = prepare_exception_embed(e);
                post_webhook(commons.client.clone(), &exception_hook).await?;
                thread::sleep(Duration::from_secs(10));
                break;
            }
            _ => {}
        }
        thread::sleep(Duration::from_millis(std::env::var("MONITOR_DELAY").unwrap_or("750".to_string()).parse().unwrap()));
    }

    Ok(())
}