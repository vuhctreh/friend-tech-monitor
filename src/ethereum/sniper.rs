use ethers::types::Address;
use ethers::types::U256;
use eyre::{eyre, Result};
use crate::ethereum::config::WalletConfig;
use crate::ethereum::contract::{call_buy_shares, get_owned_shares};

pub async fn snipe(config: &WalletConfig, address: Address, amount: &u64) -> Result<()> {

    let amount = U256::from(amount.clone());

    let  receipt = call_buy_shares(config.clone(), address, amount).await;

    log::debug!("{receipt:?}");

    match receipt {
        Some(tx) => {
            match tx.status {
                Some(x) => {
                    if x.is_zero() {
                        return Err(eyre!("Transaction failed!"));
                    }
                }
                None => {
                    return Err(eyre!("No status. Maybe the transaction was not mined?"));
                }
            }
        }
        None => {
            return Err(eyre!("Sniping aborted."));
        }
    }

    log::info!("Shares owned: {}", get_owned_shares(config.clone(), address).await?);

    Ok(())
}