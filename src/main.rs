use std::error::Error;
use crate::kosetto_api::kosetto_client;
use dotenvy::dotenv;
use reqwest::{Client, Response, StatusCode};
use crate::io_utils::json_loader::load_monitor_list;
use crate::kosetto_api::types::KosettoResponse;
use std::{thread, time::Duration};
use ethers::prelude::Address;
use crate::discord_utils::types::Webhook;
use crate::discord_utils::webhook_utils::{post_webhook, prepare_webhook};
use crate::ethereum::config::WalletConfig;
use crate::kosetto_api::kosetto_client::{find_user_in_search};

mod kosetto_api;
mod discord_utils;
mod io_utils;
mod ethereum;

// TODO: make sniper and webhook parallel.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv().expect("ERROR: Could not load .env file.");

    let webhook_url: String = std::env::var("WEBHOOK_URL")
        .expect("ERROR: WEBHOOK_URL env has not been set.");

    let delay = std::env::var("DELAY")
        .expect("ERROR: DELAY in .env was not set")
        .parse::<u64>().unwrap();

    let client: Client = Client::new();

    let config: WalletConfig = WalletConfig::new().await;

    let mut monitor_list: Vec<String> = load_monitor_list().monitor;

    loop {
        for target in monitor_list.clone().iter() {

            println!("LOG: Beginning monitor for: {}", target);

            let monitor_target = &target.clone();

            let user_info: Result<KosettoResponse, StatusCode> = kosetto_client::search_user(&client, monitor_target)
                .await;

            match user_info {
                // If user info is returned from Kosetto API
                Ok(user_info) => {

                    // Search for monitored user in endpoint -> returns None if not found
                    let res = find_user_in_search(&user_info, monitor_target.clone()).await;

                    match res {
                        // If exact match
                        Some(x) => {

                            match x.address.parse::<Address>() {
                                Ok(address) => {
                                    ethereum::sniper::snipe(config.clone(), address).await.unwrap_or(println!("ERROR: Could not snipe address"));
                                },
                                Err(e) => {
                                    println!("ERROR: Could not parse address: {}", e);}
                            }

                            // Prepare & send webhook
                            let webhook: Webhook = prepare_webhook(x);
                            let resp: Result<Response, reqwest::Error> = post_webhook(&client, &webhook_url, &webhook).await;

                            match resp {
                                // Sent webhook
                                Ok(resp) => {
                                    // Ok
                                    if resp.status() == StatusCode::OK || resp.status() == StatusCode::NO_CONTENT {
                                        println!("LOG: Successfully sent webhook for: {} with status code: {}", target, resp.status());
                                    } else {
                                        // Webhook sent but not successful
                                        println!("WARN: Failed to send webhook for: {} with status code: {}", target, resp.status());
                                    }
                                }
                                // Failed to send webhook
                                Err(e) => {
                                    println!("LOG: Failed to send webhook for: {}", target);
                                    println!("ERROR: {:?}", e);
                                }
                            }
                            // Remove target from monitor list
                            <Vec<String> as AsMut<Vec<String>>>::as_mut(&mut monitor_list).retain(|x| x != monitor_target);
                        }
                        None => {
                            println!("LOG: No match found for: {}", monitor_target);
                        }
                    }
                },
                Err(e) => {
                    println!("ERROR: status code => {}", e);
                }
            }

            println!("LOG: Sleeping for 1 second...");
            thread::sleep(Duration::from_secs(1));

        }

        println!("LOG: Finished monitoring all targets. Sleeping for 10 seconds...");
        thread::sleep(Duration::from_secs(delay));
        println!("LOG: Beginning new monitoring cycle.");
        println!("--------------------------------");
    }
}

#[tokio::test]
async fn test_ethereum() -> Result<(), Box<dyn Error>> {
    dotenv().expect("ERROR: Could not load .env file.");

    ethereum::sniper::snipe(Default::default(), Default::default()).await.expect("TODO: panic message");

    Ok(())
}