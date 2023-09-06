use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use ethers::abi::AbiDecode;
use ethers::prelude::k256::elliptic_curve::rand_core::block::BlockRngCore;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, Block, BlockNumber, Transaction, U256};
use eyre::{Result};
use reqwest::{Client, Response, StatusCode};
use crate::discord_utils::types::Webhook;
use crate::discord_utils::webhook_utils::{post_webhook, prepare_user_signup_embed};
use crate::ethereum::contract::{BuySharesCall, FriendtechSharesV1Calls};
use crate::io_utils::json_loader::{load_monitor_list, write_monitor_list};
use crate::ethereum::commons::WalletCommons;
use crate::kosetto_api::kosetto_client::get_user_by_address;
use crate::kosetto_api::types::ExactUser;
use crate::sniper::sniper::snipe;

const ADDRESS: &str = "0xcf205808ed36593aa40a44f10c7f7c2f67d4a4d4";

pub async fn monitor_v2(commons: &WalletCommons, block_number: BlockNumber) -> Result<Option<()>> {
    let provider: Provider<Http> = commons.provider.clone();

    let previous_block: Option<Block<Transaction>> = get_previous_block_txs(&provider,  block_number).await?;

    match previous_block {
        Some(block) => {
            let previous_block_txs: Vec<Transaction> = block.transactions;

            let filtered_transactions: Vec<BuySharesCall> = filter_signup_txs(previous_block_txs)?;

            if filtered_transactions.len() > 0 {
                log::info!("Buys in block: {}", filtered_transactions.len());

                for tx in filtered_transactions {
                    tokio::spawn(async move {
                        let mut monitor_map: HashMap<String, u64> = load_monitor_list().unwrap();

                        if monitor_map.len() == 0usize {
                            log::warn!("monitor.json is empty. Bot will continue running in case there are snipes ongoing.");
                        }

                        let thread_client: Client = Client::new();

                        let user_data: Result<ExactUser> = resolve_user_by_address(&thread_client, tx.shares_subject.clone()).await;

                        match user_data {
                            Ok(data) => {
                                let monitored_users = monitor_map.clone();

                                // Sniper initialisation
                                if monitored_users.contains_key(&data.twitter_username) {
                                    let amount = monitored_users.get(&data.twitter_username).unwrap().clone();

                                    log::info!("Monitored user found: {}", &data.twitter_username);
                                    let thread_data = data.clone();
                                    tokio::spawn(async move {
                                        log::info!("Sending snipe transaction.");

                                        let address: Address = Address::from_str(&*thread_data.address.clone()).unwrap();
                                        let snipe_commons: WalletCommons = WalletCommons::new().unwrap();
                                        let _ = snipe(snipe_commons, address, amount).await;
                                    });
                                }

                                let webhook: Webhook = prepare_user_signup_embed(data.clone());
                                let _resp: Response = post_webhook(&thread_client,  &webhook).await.unwrap();

                                monitor_map.remove(&*data.twitter_username);
                                write_monitor_list(monitor_map);

                            }
                            Err(e) => {
                                log::error!("Failed to resolve user with address: {}, {}", tx.clone(), e);
                            }
                        }
                    });
                }
            }
            Ok(Some(()))
        }
        None => return Ok(None)
    }
}

pub async fn get_previous_block_txs(provider: &Provider<Http>, block_number: BlockNumber) -> Result<Option<Block<Transaction>>> {
    let transactions: Option<Block<Transaction>> = provider.get_block_with_txs(block_number).await?;

    match transactions {
        Some(block) => Ok(Some(block)),
        None => Ok(None)
    }
}

// TODO: make sure transactions did not revert.
pub fn filter_signup_txs(txs: Vec<Transaction>) -> Result<Vec<BuySharesCall>> {
    let friend_tech_address: Address = Address::from_str(ADDRESS)?;

    let mut filtered_txs: Vec<BuySharesCall> = vec![];

    for tx in txs {
        match tx.to {
            Some(address) => {
                if address == friend_tech_address && tx.value == U256::zero() {
                    let data = FriendtechSharesV1Calls::decode(&tx.input)?;

                    match data {
                        FriendtechSharesV1Calls::BuyShares(x) => {
                            if x.amount == U256::one() {
                                filtered_txs.push(x);
                            }
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(filtered_txs)
}

pub async fn resolve_user_by_address(client: &Client, address: Address) -> Result<ExactUser> {
    let mut user: Response = get_user_by_address(client, address).await?;
    let secondary_delay: u64= std::env::var("SECONDARY_DELAY").unwrap().parse().unwrap();

    match user.status() {
        StatusCode::OK => {}
        StatusCode::BAD_GATEWAY => {
            for i in 0..19 {
                if user.status() != StatusCode::BAD_GATEWAY { break;}
                log::warn!("502 error getting user. Retrying...");
                user = get_user_by_address(&client, address).await?;
                thread::sleep(std::time::Duration::from_millis(secondary_delay));
                if i == 18 {
                    log::warn!("Exceeded max retries. Aborting.")
                }
            }
        }
        StatusCode::NOT_FOUND => {
            for i in 0..19 {
                if user.status() != StatusCode::NOT_FOUND { break; }
                log::warn!("404 error getting user. Retrying...");
                user = get_user_by_address(client, address).await?;
                thread::sleep(std::time::Duration::from_millis(secondary_delay));
                if i == 18 {
                    log::warn!("Exceeded max retries. Aborting.")
                }
            }
        }
        _ => {
            log::error!("Error: {}", user.status());
        }
    }

    Ok(serde_json::from_str(&user.text().await?)?)
}