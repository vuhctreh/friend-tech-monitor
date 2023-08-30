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
use crate::io_utils::json_loader::load_monitor_list;
use crate::ethereum::commons::WalletCommons;
use crate::kosetto_api::kosetto_client::get_user_by_address;
use crate::kosetto_api::types::ExactUser;

const ADDRESS: &str = "0xcf205808ed36593aa40a44f10c7f7c2f67d4a4d4";

// TODO: delays
pub async fn monitor_v2(commons: &WalletCommons, block_number: BlockNumber) -> Result<()> {
    let provider: Provider<Http> = commons.provider.clone();

    // let monitor_map: HashMap<String, u64> = load_monitor_list()?;
    //
    // if monitor_map.len() == 0usize {
    //     log::warn!("monitor.json is empty. Bot will continue running in case there are snipes ongoing.");
    // }
    //
    // let mut _new_map: HashMap<String, u64> = HashMap::new();

    let previous_block_txs: Option<Vec<Transaction>> = get_previous_block_txs(&provider,  block_number).await?;

    let filtered_transactions: Vec<BuySharesCall> = match previous_block_txs {
        Some(txs) => {
            filter_signup_txs(txs)?
        },
        None => vec![]
    };

    if filtered_transactions.len() > 0 {
        log::info!("Buys in block: {}", filtered_transactions.len());

        for tx in filtered_transactions {
            tokio::spawn(async move {
                let thread_client: Client = Client::new();

                let user_data: Result<ExactUser> = resolve_user_by_address(&thread_client, tx.shares_subject.clone()).await;

                match user_data {
                    Ok(data) => {
                        // tokio::spawn(async move {});
                        let webhook: Webhook = prepare_user_signup_embed(data);
                        let _resp: Response = post_webhook(&thread_client,  &webhook).await.unwrap();
                    }
                    Err(e) => {
                        log::error!("Failed to resolve user with address: {}, {}", tx.clone(), e);
                    }
                }
            });
            // if monitor_map.contains_key(&user_data.address) {
            //     let webhook: Webhook = prepare_user_signup_embed(matching_user);
            //     let resp: Response = post_webhook(&client,  &webhook).await?;
            // }
        }
    }

    Ok(())
}

pub async fn get_previous_block_txs(provider: &Provider<Http>, block_number: BlockNumber) -> Result<Option<Vec<Transaction>>> {
    let transactions: Option<Block<Transaction>> = provider.get_block_with_txs(block_number).await?;

    match transactions {
        Some(block) => Ok(Some(block.transactions)),
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
                            filtered_txs.push(x);
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

    match user.status() {
        StatusCode::OK => {}
        StatusCode::BAD_GATEWAY => {
            while user.status() == StatusCode::BAD_GATEWAY {
                log::warn!("502 error getting user. Retrying...");
                user = get_user_by_address(&client, address).await?;
                thread::sleep(std::time::Duration::from_millis(350));
            }
        }
        StatusCode::NOT_FOUND => {
            while user.status() == StatusCode::NOT_FOUND {
                log::warn!("404 error getting user. Retrying...");
                user = get_user_by_address(client, address).await?;
                thread::sleep(std::time::Duration::from_millis(350));
            }
        }
        _ => {
            log::error!("Error: {}", user.status());
        }
    }

    Ok(serde_json::from_str(&user.text().await?)?)
}