use ethers::types::Address;
use ethers::types::U256;
use crate::ethereum::config::WalletConfig;
use crate::ethereum::contract::{call_buy_shares, get_owned_shares};

pub async fn snipe(config: &WalletConfig, address: Address, amount: &u64) -> Result<(), String> {

    let amount = U256::from(amount.clone());

    let  receipt = call_buy_shares(config.clone(), address, amount).await;

    log::debug!("{receipt:?}");

    match receipt.status {
        Some(status) => {
            if status.is_zero() {
                return Err(String::from("WARN: Transaction failed."));
            }
        }
        None => {
            return Err("ERROR: No status".to_string());
        }
    }

    log::info!("Shares owned: {}", get_owned_shares(config.clone(), address).await?);

    Ok(())
}