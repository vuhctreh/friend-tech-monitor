use std::sync::Arc;

use ethers::{
    core::types::{Address},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
};
use crate::ethereum::contract::call_buy_shares;

// 0x385D77E5b0D5D97640135c1a0F2F7702619cfaB3 -> banana wallet
pub async fn test() -> Result<String, String> {

    let contract_address = "0xEeaA6B7290F35D588072272E75f1D5eA57827f4f".parse::<Address>().expect("Could not parse contract");

    let provider = Arc::new({

        let provider = Provider::<Http>::try_from(
            std::env::var("RPC_URL").expect("RPC not set in .env.")
        ).expect("Invalid RPC URL.");

        let chain_id = provider.get_chainid().await.expect("Failed to get chain id.");

        let wallet = std::env::var("PRIVATE_KEY")
            .expect("PRIVATE_KEY not set in.env.")
            .parse::<LocalWallet>().expect("Invalid private key.")
            .with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(provider, wallet)
    });

    let address_to_buy = "0x385D77E5b0D5D97640135c1a0F2F7702619cfaB3".parse().expect("Could not parse address");

    let  receipt = call_buy_shares(provider, address_to_buy, 1.into()).await;

    println!("{receipt:?}");

    match receipt.status {
        Some(status) => {
            if status.is_zero() {
                Err("Transaction failed.")
            } else {
                Ok("Transaction successful.")
            }
        },
        None => Err("No status")
    }.expect("TODO: panic message");


    // let balance = contract.shares_balance(address_to_buy.clone(), address_to_buy.clone())
    //     .call()
    //     .await
    //     .expect("bruh");

    // println!("{balance:?}");

    Ok("Hello World".to_string())
}