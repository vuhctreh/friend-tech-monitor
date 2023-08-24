use std::collections::HashMap;
use std::{env, thread};
use std::env::VarError;
use std::time::Duration;
use ethers::types::Address;
use reqwest::{Client, Response, StatusCode};
use eyre::Result;
use crate::auth::sms::auth_impl::generate_auth_token;
use crate::discord_utils::types::Webhook;
use crate::discord_utils::webhook_utils::{post_webhook, prepare_user_signup_embed};
use crate::{sniper};
use crate::ethereum::config::WalletConfig;
use crate::io_utils::json_loader::{load_monitor_list, write_monitor_list};
use crate::kosetto_api::kosetto_client;
use crate::kosetto_api::kosetto_client::find_user_in_search;
use crate::kosetto_api::types::{KosettoResponse, User};

pub async fn monitor(client: Client, config: WalletConfig, delay: u64) -> Result<()> {

    let monitor_map: HashMap<String, u64> = load_monitor_list()?;

    if monitor_map.len() == 0usize {
        log::warn!("monitor.json is empty. Bot will continue running in case there are snipes ongoing.");
    }

    let mut new_map: HashMap<String, u64> = HashMap::new();

    for (key, value) in monitor_map.iter() {
        log::info!("Beginning monitor for: {}", key);
        let monitor_target: &String = &key.clone();

        let load_auth_token: Result<String, VarError> = env::var("AUTH_TOKEN");

        let mut token: String = String::new();

        match load_auth_token {
            Ok(_) => {
                token = load_auth_token.unwrap();
            }
            Err(_) => {
                token = generate_auth_token(&client).await?;
                // TODO: replace token in .env
                env::set_var("AUTH_TOKEN", &token);
            }
        }

        let resp: Response = kosetto_client::search_user(&client, monitor_target, token)
            .await?;

        match resp.status() {
            StatusCode::OK => {
                let json: KosettoResponse = serde_json::from_str(&resp.text().await?)?;
                parse_response(config.clone(), json, key.clone(), client.clone()).await?
            }
            StatusCode::NOT_FOUND => {
                log::info!("No users returned from search.");
                new_map.insert(key.clone(), *value);
            }
            StatusCode::UNAUTHORIZED => {
                log::warn!("Token expired. Refreshing.");

                let new_token = generate_auth_token(&client).await?;

                env::set_var("AUTH_TOKEN", new_token);

                new_map.insert(key.clone(), *value);
            }
            _ => {
                log::error!("Unexpected status code: {}. Will retry next cycle.", &resp.status());
                new_map.insert(key.clone(), *value);
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    write_monitor_list(new_map)?;

    log::info!("Finished monitoring.");
    log::info!("Sleeping for: {}s", delay);
    thread::sleep(Duration::from_secs(delay));

    log::info!("-----------------------------");

    Ok(())
}

async fn parse_response(config: WalletConfig, response: KosettoResponse, target: String, client: Client) -> Result<()> {
    let res: Option<User> = find_user_in_search(&response, &target);

    match res {
        Some(matching_user) => {
            match matching_user.address.parse::<Address>() {
                Ok(address) => {
                    tokio::spawn( async move {
                        let snipe_result = sniper::sniper::snipe(config, address).await;

                        match snipe_result {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("Sniper failed with error: {}", e);
                            }
                        }
                    });
                },
                Err(e) => {
                    log::error!("Could not parse address: {}", e);
                }
            }

            // Prepare & send webhook
            let webhook: Webhook = prepare_user_signup_embed(matching_user);
            let resp: Response = post_webhook(&client,  &webhook).await?;

            if resp.status() == StatusCode::OK || resp.status() == StatusCode::NO_CONTENT {
                log::info!("Successfully sent webhook for: {} with status code: {}", &target, resp.status());
            } else {
                log::warn!("Failed to send webhook for: {} with status code: {}", &target, resp.status());
            }
        }
        None => {
            log::info!("No match found for: {}", &target);
        }

    }

    Ok(())
}