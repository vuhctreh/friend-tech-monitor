use ethers::prelude::TransactionReceipt;
use ethers::types::Address;
use ethers::types::U256;
use eyre::{eyre, Result};
use crate::ethereum::config::WalletConfig;
use crate::ethereum::contract::{call_buy_shares, get_owned_shares};

pub async fn snipe(config: &WalletConfig, address: Address, amount: &u64) -> Result<()> {

    let amount: U256 = U256::from(amount.clone());

    let receipt: TransactionReceipt = call_buy_shares(config.clone(), address, amount).await?;

    log::debug!("{receipt:?}");

    match receipt.status {
        Some(status) => {
            if status.is_zero() {
                return Err(eyre!("Transaction failed!"));
            }
        },
        None => {
            return Err(eyre!("No transaction status. Maybe the transaction was not sent?"));
        }
    }

    log::info!("Shares owned: {}", get_owned_shares(config.clone(), address).await?);

    Ok(())
}