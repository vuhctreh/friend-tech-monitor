use ethers::types::Address;
use crate::ethereum::config::WalletConfig;
use crate::ethereum::contract::{call_buy_shares, get_owned_shares};

pub async fn snipe(config: WalletConfig, address: Address) -> Result<(), String> {
    let  receipt = call_buy_shares(config.clone(), address, 1.into()).await;

    println!("{receipt:?}");

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

    println!("Shares owned: {}", get_owned_shares(config.clone(), address).await);

    Ok(())
}