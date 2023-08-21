use std::error::Error;
use crate::kosetto_api::kosetto_client;
use dotenvy::dotenv;
use reqwest::{Client, StatusCode};
use crate::io_utils::json_loader::load_monitor_list;
use crate::kosetto_api::types::KosettoResponse;
use std::{thread, time::Duration};
use crate::kosetto_api::kosetto_client::monitor;

mod kosetto_api;
mod discord_utils;
mod io_utils;
mod ethereum;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv().expect("ERROR: Could not load .env file.");

    let webhook_url: String = std::env::var("WEBHOOK_URL")
        .expect("ERROR: WEBHOOK_URL env has not been set.");

    let delay = std::env::var("DELAY")
        .expect("ERROR: DELAY in .env was not set")
        .parse::<u64>().unwrap();

    let client: Client = Client::new();

    let mut monitor_list: Vec<String> = load_monitor_list().monitor;

    loop {
        for target in monitor_list.clone().iter() {

            println!("LOG: Beginning monitor for: {}", target);

            let monitor_target = &target.clone();

            let user_info: Result<KosettoResponse, StatusCode> = kosetto_client::get_user(&client, monitor_target)
                .await;

            match user_info {
                Ok(user_info) => {
                    let res = monitor(&user_info, monitor_target.clone(), &client, &webhook_url).await;
                    match res {
                        Ok(1) => {
                            <Vec<String> as AsMut<Vec<String>>>::as_mut(&mut monitor_list).retain(|x| x != monitor_target);
                        },
                        Ok(2) => {
                            println!("LOG: Exact match not found. Continuing...");
                        },
                        Err(e) => {
                            println!("ERROR: {}", e);
                        }
                        _ => {}
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

    ethereum::wallet::test().await.expect("TODO: panic message");

    Ok(())
}