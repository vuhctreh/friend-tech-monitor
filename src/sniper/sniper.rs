use std::thread;
use ethers::prelude::TransactionReceipt;
use ethers::types::U256;
use ethers::types::Address;
use crate::ethereum::config::WalletConfig;
use crate::sniper::sniper_contract_logic::{prepare_snipe, send_snipe_transaction};
use crate::ethereum::config::{Contract};
use eyre::Result;

/// Snipes shares for a given address when available.
/// Uses the contract to determine the price of a share.
/// If the price is 0, will keep getting the price until
/// p > 0. If the price is above the limit, will bubble an error.
pub async fn snipe(config: WalletConfig, address: Address) -> Result<()> {
    let contract: Contract = config.contract.clone();

    log::info!("Preparing to snipe.");
    log::info!("Shares address: {}", &address);

    let mut transaction_value: U256 = prepare_snipe(&contract, address).await?;

    // TODO: make delay configurable
    // Updates transaction_value while it is 0 (user has not bought their first share yet)
    while transaction_value.is_zero() {
        log::warn!("Transaction value is 0. Waiting for user to buy their first share.");
        transaction_value = prepare_snipe(&contract, address).await?;
        thread::sleep(std::time::Duration::from_millis(200));
    }

    log::info!("Ready to snipe.");

    let receipt: TransactionReceipt = send_snipe_transaction(contract, address, transaction_value).await?;

    match receipt.status {
        Some(x) => {
            match x.as_u64() {
                0 => {
                    log::warn!("Transaction reverted -> Status: 0.");
                },
                1 => {
                    log::info!("Transaction successful -> Status: 1.");
                },
                y => {
                    log::info!("Transaction included with unexpected status: {}.", y);
                },
            }
        }
        None => {}
    }

    Ok(())
}