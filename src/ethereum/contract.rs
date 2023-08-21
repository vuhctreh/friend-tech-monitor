use std::thread;
use ethers::core::types::{Address, U256};
use ethers::prelude::TransactionReceipt;
use crate::ethereum::config::WalletConfig;

pub async fn call_buy_shares(config: WalletConfig, buy_address: Address, amount: U256) -> TransactionReceipt {
    let contract = config.contract.clone();

    let mut transaction_value: U256 = contract.get_buy_price_after_fee(buy_address.clone(), amount.clone()).await.unwrap();

    while transaction_value == U256::zero() {
        log::info!("Waiting for user to buy shares...");
        transaction_value = contract.get_buy_price(buy_address.clone(), amount.clone()).await.unwrap();
        thread::sleep(std::time::Duration::from_millis(500));
    }

    log::info!("Current price of shares: {}", &transaction_value);

    log::info!("Purchase transaction sent...");

    let transaction = contract.buy_shares(buy_address.clone(), amount)
        .gas(150000)
        .value(&transaction_value)
        .send()
        .await
        .expect("Failed to buy shares.")
        .await
        .expect("Failed to buy shares.")
        .unwrap();

    match transaction.status {
        Some(x) => {
            match x.as_u64() {
                0 => {
                    log::warn!("Transaction reverted -> Status: 0");
                },
                1 => {
                    log::info!("Transaction successful -> Status: 1");
                },
                y => {
                    log::info!("Transaction included with unexpected status: {}", y);
                },
            }
        }
        None => {}
    }

    transaction
}

// TODO: remove expect
pub async fn get_owned_shares(config: WalletConfig, address: Address) -> U256 {
    config.contract.clone().shares_balance(address, config.wallet_address.clone())
        .call()
        .await
        .expect("ERROR: Could not get shares balance.")
}