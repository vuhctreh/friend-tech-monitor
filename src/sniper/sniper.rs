//! Sniper implementation.

use std::{env, thread};
use std::env::VarError;
use std::str::FromStr;
use ethers::prelude::TransactionReceipt;
use ethers::types::U256;
use ethers::types::Address;
use ethers::utils::parse_ether;
use crate::ethereum::commons::WalletCommons;
use crate::sniper::sniper_contract_logic::{get_owned_shares, prepare_snipe, send_snipe_transaction};
use crate::ethereum::commons::{Contract};
use eyre::{Result, eyre};

/// FIXED: Will send buyShares transaction with a fixed value every time. More reliable, faster,
/// but friend.tech contract does not refund overpay. In the future, it may be interesting to use a
/// custom contract.
///
/// DYNAMIC: Checks the price of shares and send the exact value. Less reliable, slower, lower chance of
/// overpay.
enum SniperMode {
    FIXED,
    DYNAMIC
}

impl FromStr for SniperMode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "FIXED" => Ok(SniperMode::FIXED),
            "DYNAMIC" => Ok(SniperMode::DYNAMIC),
            _ => Err(format!("Invalid sniper mode: {}", s))
        }
    }
}

// TODO: gas.
/// Snipes shares for a given address.
pub async fn snipe(commons: WalletCommons, address: Address) -> Result<()> {
    let contract: Contract = commons.contract.clone();

    let mode: Result<SniperMode, String> = SniperMode::from_str(&*env::var("SNIPER_MODE")?);

    return match mode {
        Ok(SniperMode::FIXED) => {
            let price: U256 = parse_ether(env::var("SNIPE_PRICE")?)?;

            log::info!("Sniping at fixed price: {}", &price);

            let receipt: TransactionReceipt = send_snipe_transaction(contract, address, price).await?;

            validate_transaction(receipt);

            Ok(())
        }
        Ok(SniperMode::DYNAMIC) => {
            log::info!("Preparing to snipe.");
            log::info!("Shares address: {}", &address);

            let mut transaction_value: Result<U256> = prepare_snipe(&contract, address).await;

            match transaction_value {
                Ok(price) => {
                    log::info!("Ready to snipe.");
                    let receipt: TransactionReceipt = send_snipe_transaction(contract, address, price).await?;
                    validate_transaction(receipt);
                },
                Err(e) => {
                    log::warn!("{}", e);
                }
            }

            Ok(())
        }
        Err(e) => {
            log::error!("{}", e);
            Err(eyre!(e))
        }
    }
}

fn validate_transaction(receipt: TransactionReceipt) {
    match receipt.status {
        Some(x) => {
            match x.as_u64() {
                0 => {
                    log::warn!("Transaction reverted -> Status: 0.");
                },
                1 => {
                    log::info!("Transaction successful -> Status: 1.");
                    log::info!("Transaction hash: {}", receipt.transaction_hash);
                },
                y => {
                    log::info!("Transaction included with unexpected status: {}.", y);
                },
            }
        }
        None => {}
    }
}