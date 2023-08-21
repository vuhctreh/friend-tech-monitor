use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use ethers::types::Address;
use reqwest::{Client, Error, Response, StatusCode};
use crate::discord_utils::types::Webhook;
use crate::discord_utils::webhook_utils::{post_webhook, prepare_webhook};
use crate::ethereum;
use crate::ethereum::config::WalletConfig;
use crate::io_utils::json_loader::{load_monitor_list, write_monitor_list};
use crate::kosetto_api::kosetto_client;
use crate::kosetto_api::kosetto_client::find_user_in_search;
use crate::kosetto_api::types::{KosettoResponse};

// TODO: clean this up
pub async fn monitor(client: Client, config: WalletConfig, delay: u64) -> Result<String, String> {

    let monitor_map: HashMap<String, u64> = load_monitor_list();

    if load_monitor_list().len() == 0 {
        panic!("Monitor list is empty");
    }

    let mut new_map: HashMap<String, u64> = HashMap::new();

    for (key, value) in monitor_map.iter() {
        log::info!("Beginning monitor for: {}", key);
        let monitor_target = &key.clone();

        let user_info: Result<KosettoResponse, StatusCode> = kosetto_client::search_user(&client, monitor_target)
            .await;

        match user_info {
            Ok(user_info) => {
                let bruh = parse_response(config.clone(), user_info, key.clone(), value.clone(), client.clone()).await;
                if bruh.is_err() { log::error!("{:?}", bruh) };
            }
            Err(StatusCode::NOT_FOUND) => {
                log::info!("No users returned from search.");
                new_map.insert(monitor_target.clone(), *value);
            }
            Err(e) => {
                log::error!("User search failed with status code: {}", e);
                new_map.insert(monitor_target.clone(), *value);
            }
        }

        thread::sleep(Duration::from_secs(2));
    }

    write_monitor_list(new_map).expect("TODO: panic message");

    log::info!("Sleeping for: {}s", delay);
    thread::sleep(Duration::from_secs(delay));

    Ok("h".to_string())
}

// TODO: clean this up
// Errors: 1 =
async fn parse_response(config: WalletConfig, response: KosettoResponse, target: String, amount: u64, client: Client) -> Result<(), Error> {
    // Search for monitored user in endpoint -> returns None if not found
    let res = find_user_in_search(&response, &target);

    match res {
        Some(matching_user) => {
            match matching_user.address.parse::<Address>() {
                Ok(address) => {
                    let snipe_result = ethereum::sniper::snipe(&config, address, &amount).await;

                    match snipe_result {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("Sniper failed with error: {}", e);
                        }
                    }
                },
                Err(e) => {
                    log::error!("Could not parse address: {}", e);
                }
            }

            // Prepare & send webhook
            let webhook: Webhook = prepare_webhook(matching_user);
            let resp: Result<Response, Error> = post_webhook(&client,  &webhook).await;

            match resp {
                Ok(response) => {
                    if response.status() == StatusCode::OK || response.status() == StatusCode::NO_CONTENT {
                        log::info!("Successfully sent webhook for: {} with status code: {}", &target, response.status());
                    } else {
                        // Webhook sent but not successful
                        log::warn!("Failed to send webhook for: {} with status code: {}", &target, response.status());
                    }
                }
                // Failed to send webhook
                Err(e) => {
                    log::warn!("Failed to send webhook for: {}", &target);
                    log::error!("{:?}", e);
                }
            }

        }
        None => {
            log::info!("No match found for: {}", &target);
        }
    }

    Ok(())
}