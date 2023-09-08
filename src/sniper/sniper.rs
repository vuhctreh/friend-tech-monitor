//! Sniper implementation.

use std::{env, thread};
use std::env::VarError;
use std::str::FromStr;
use ethers::prelude::TransactionReceipt;
use ethers::types::U256;
use ethers::types::Address;
use ethers::utils::parse_ether;
use crate::ethereum::commons::WalletCommons;
use crate::sniper::sniper_contract_logic::{send_snipe_transaction};
use crate::ethereum::commons::{Contract};
use eyre::{Result, eyre};

// TODO: gas.
/// Snipes shares for a given address using custom contract.
pub async fn snipe(commons: WalletCommons, address: Address, amount: u64) -> Result<()> {
    let contract: Contract = commons.contract.clone();

    let receipt: TransactionReceipt = send_snipe_transaction(contract, address, U256::from(amount)).await?;

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

    Ok(())
}