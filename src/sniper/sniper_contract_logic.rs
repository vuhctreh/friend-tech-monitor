//! Wraps contract calls for use in the sniper.

use ethers::types::{Address, TransactionReceipt, U256};
use ethers::utils::parse_ether;
use eyre::{eyre, Result};
use crate::ethereum::commons::{Contract, ApplicationCommons};

/// Calls the magical_buy_shares function on the custom sniper contract.
pub async fn send_snipe_transaction(contract: Contract, address: Address, amount: U256) -> Result<TransactionReceipt> {
    log::info!("Sending transaction...");

    let transaction = contract.magical_buy_shares(address, amount)
        .gas(150000)
        .send()
        .await?
        .await?;

    match transaction {
        Some(tx) => {
            Ok(tx)
        }
        None => {
            return Err(eyre!("Transaction was not sent."));
        }
    }
}