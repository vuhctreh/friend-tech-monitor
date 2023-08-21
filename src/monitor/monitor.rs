use std::thread;
use std::time::Duration;
use ethers::types::Address;
use reqwest::{Client, Response, StatusCode};
use crate::discord_utils::types::Webhook;
use crate::discord_utils::webhook_utils::{post_webhook, prepare_webhook};
use crate::ethereum;
use crate::ethereum::config::WalletConfig;
use crate::io_utils::json_loader::load_monitor_list;
use crate::kosetto_api::kosetto_client;
use crate::kosetto_api::kosetto_client::find_user_in_search;
use crate::kosetto_api::types::KosettoResponse;

pub async fn monitor(client: Client, config: WalletConfig, webhook_url: &String, delay: u64) -> Result<String, String> {
    let mut monitor_list: Vec<String> = load_monitor_list().monitor;

    loop {
        for target in monitor_list.clone().iter() {

            log::info!("Beginning monitor for: {}", target);

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
                                    ethereum::sniper::snipe(config.clone(), address).await.unwrap_or(log::error!("Failed to snipe {}", address));
                                },
                                Err(e) => {
                                    log::error!("Could not parse address: {}", e);
                                }
                            }

                            // Prepare & send webhook
                            let webhook: Webhook = prepare_webhook(x);
                            let resp: Result<Response, reqwest::Error> = post_webhook(&client, &webhook_url, &webhook).await;

                            match resp {
                                // Sent webhook
                                Ok(resp) => {
                                    // Ok
                                    if resp.status() == StatusCode::OK || resp.status() == StatusCode::NO_CONTENT {
                                        log::info!("Successfully sent webhook for: {} with status code: {}", target, resp.status());
                                    } else {
                                        // Webhook sent but not successful
                                        log::warn!("Failed to send webhook for: {} with status code: {}", target, resp.status());
                                    }
                                }
                                // Failed to send webhook
                                Err(e) => {
                                    log::warn!("Failed to send webhook for: {}", target);
                                    log::error!("{:?}", e);
                                }
                            }
                            // Remove target from monitor list
                            <Vec<String> as AsMut<Vec<String>>>::as_mut(&mut monitor_list).retain(|x| x != monitor_target);
                        }
                        None => {
                            log::info!("No match found for: {}", monitor_target);
                        }
                    }
                },
                Err(e) => {
                    log::error!("status code => {}", e);
                }
            }

            log::info!("Sleeping for 1 second...");
            thread::sleep(Duration::from_secs(1));
        }

        log::info!("Finished monitoring all targets. Sleeping for 10 seconds...");
        thread::sleep(Duration::from_secs(delay));
        log::info!("Beginning new monitoring cycle.");
        println!("--------------------------------");
    }
}