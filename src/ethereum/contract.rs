use std::thread;
use ethers::core::types::{Address, U256};
use ethers::prelude::{TransactionReceipt};
use ethers::utils::{parse_ether};
use eyre::{eyre, Result};
use crate::ethereum::config::WalletConfig;

pub async fn call_buy_shares(config: WalletConfig, buy_address: Address, amount: U256) -> Result<TransactionReceipt> {
    let contract = config.contract.clone();

    let mut transaction_value: U256 = contract.get_buy_price_after_fee(buy_address.clone(), amount.clone()).await.unwrap();

    let limit_env = std::env::var("LIMIT_PRICE");

    let mut limit: U256 = U256::zero();

    match limit_env {
        Ok(limit_as_string) => {
            let parsed_limit = parse_ether(limit_as_string);

            match parsed_limit {
                Ok(x) => {
                    limit = x;
                }
                Err(e) => {
                    return Err(eyre!("Invalid limit price in.env: {}", e));
                }
            }

            log::info!("Limit price: {}", limit);
        }
        Err(_) => {
            return Err(eyre!("Limit price not set in .env."));
        }
    }

    if transaction_value > limit {
        return Err(eyre!("Transaction value is greater than limit price."));
    }

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
        .await?
        .await?
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

    Ok(transaction)
}

pub async fn get_owned_shares(config: WalletConfig, address: Address) -> Result<U256> {
    let contract_response = config.contract.clone().shares_balance(address, config.wallet_address.clone())
        .call()
        .await;

    match contract_response {
        Ok(_) => Ok(contract_response.unwrap()),
        Err(e) => Err(eyre!(e))
    }
}