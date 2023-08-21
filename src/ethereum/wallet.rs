use crate::ethereum::config::WalletConfig;
use crate::ethereum::contract::{call_buy_shares, get_owned_shares};

// 0x385D77E5b0D5D97640135c1a0F2F7702619cfaB3 -> banana wallet
pub async fn test() -> Result<(), ()> {

    let config: WalletConfig = WalletConfig::new().await;

    let address_to_buy = "0x385D77E5b0D5D97640135c1a0F2F7702619cfaB3".parse().expect("Could not parse address");

    let  receipt = call_buy_shares(config.clone(), address_to_buy, 1.into()).await;

    println!("{receipt:?}");

    match receipt.status {
        Some(status) => {
            if status.is_zero() {
                Err("Transaction failed.")
            } else {
                println!("Shares owned: {}", get_owned_shares(config.clone(), address_to_buy).await);
                Ok(())
            }
        },
        None => Err("No status")
    }.expect("TODO: panic message");

    Ok(())
}