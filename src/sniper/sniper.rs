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
pub async fn snipe(config: WalletCommons, address: Address) -> Result<()> {
    let contract: Contract = config.contract.clone();

    log::info!("Preparing to snipe.");
    log::info!("Shares address: {}", &address);

    let mut transaction_value: U256 = prepare_snipe(&contract, address).await?;

    // Updates transaction_value while it is 0 (user has not bought their first share yet)
    while transaction_value.is_zero() {
        // Delay between monitor cycles (default 200ms or 0.2s)
        let delay = env::var("SNIPER_DELAY").unwrap_or("200".to_string()).parse::<u64>()?;
        log::warn!("Transaction value is 0. Waiting for {} to buy their first share.", &address);

        transaction_value = prepare_snipe(&contract, address).await?;
        thread::sleep(std::time::Duration::from_millis(delay));
    }

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

            let owned_shares = get_owned_shares(config, user_address).await?;
            log::info!("Owned shares: {}", owned_shares);
        }
        Err(_) => {
            log::error!("Could not parse address in env.");
        }
    }

    Ok(())
}