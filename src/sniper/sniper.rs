//! Sniper implementation.

use std::{env, thread};
use std::env::VarError;
use ethers::prelude::TransactionReceipt;
use ethers::types::U256;
use ethers::types::Address;
use crate::ethereum::commons::WalletCommons;
use crate::sniper::sniper_contract_logic::{get_owned_shares, prepare_snipe, send_snipe_transaction};
use crate::ethereum::commons::{Contract};
use eyre::Result;

/// Snipes shares for a given address when available.
/// Uses the contract to determine the price of a share.
/// If the price is 0, will keep getting the price until
/// p > 0. If the price is above the limit, will bubble an error.
pub async fn snipe(commons: WalletCommons, address: Address) -> Result<()> {
    let contract: Contract = commons.contract.clone();

    log::info!("Preparing to snipe.");
    log::info!("Shares address: {}", &address);

    let mut transaction_value: U256 = prepare_snipe(&contract, address).await?;

    log::info!("Ready to snipe.");

    let receipt: TransactionReceipt = send_snipe_transaction(contract, address, transaction_value).await?;

    match receipt.status {
        Some(x) => {
            match x.as_u64() {
                0 => {
                    log::warn!("Transaction reverted -> Status: 0.");
                },
                1 => {
                    log::info!("Transaction successful -> Status: 1.");
                    log::info!("Transaction hash: {}", receipt.transaction_hash);
                },
                y => {
                    log::info!("Transaction included with unexpected status: {}.", y);
                },
            }
        }
        None => {}
    }

    let user_address_env: Result<String, VarError> = env::var("WALLET_ADDRESS");

    match user_address_env {
        Ok(x) => {
            let user_address: Address = x.parse::<Address>()?;

            let owned_shares = get_owned_shares(commons, user_address).await?;
            log::info!("Owned shares: {}", owned_shares);
        }
        Err(_) => {
            log::error!("Could not parse address in env.");
        }
    }

    Ok(())
}