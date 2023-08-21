use ethers::core::types::{Address, U256};
use ethers::prelude::TransactionReceipt;
use crate::ethereum::config::WalletConfig;

pub async fn call_buy_shares(config: WalletConfig, buy_address: Address, amount: U256) -> TransactionReceipt {
    let contract = config.contract.clone();

    let transaction_value: U256 = contract.get_buy_price(buy_address.clone(), amount.clone()).await.unwrap();

    println!("Current price of shares: {}", &transaction_value);

    println!("Attempting purchase...");

    let transaction = contract.buy_shares(buy_address.clone(), amount)
        .gas(300000)
        .value(&transaction_value)
        .send()
        .await
        .expect("Failed to buy shares.")
        .await
        .expect("Failed to buy shares.")
        .unwrap();

    println!("Transaction status: {:?}", &transaction.status);

    match transaction.status {
        Some(x) => {
            match x.as_u64() {
                0 => {
                    println!("WARN: Transaction reverted -> Status: 0");
                },
                1 => {
                    println!("LOG: Transaction successful -> Status: 1");
                },
                y => {
                    println!("LOG: Transaction included with unexpected status: {}", y);
                },
            }
        }
        None => {}
    }

    transaction
}

pub async fn get_owned_shares(config: WalletConfig, address: Address) -> U256 {
    //let contract = config.contract.clone();

    config.contract.clone().shares_balance(address, config.wallet_address.clone())
        .call()
        .await
        .expect("bruh")
}