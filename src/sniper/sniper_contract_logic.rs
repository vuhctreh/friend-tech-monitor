use ethers::types::{Address, TransactionReceipt, U256};
use ethers::utils::parse_ether;
use eyre::{eyre, Result};
use crate::ethereum::config::{Contract};

/// Checks that the value of shares is below the limit.
/// Returns a Result<U256> if the value is below the limit.
pub async fn prepare_snipe(contract: &Contract, address: Address) -> Result<U256> {
    let limit: U256 = parse_ether(std::env::var("LIMIT_PRICE")?)?;

    // TODO: amount hard coded to 1 for now, rework json to include limit
    let mut transaction_value: U256 = contract.get_buy_price_after_fee(address, U256::from(1)).await?;

    log::info!("Limit price: {}", limit);
    log::info!("Projected transaction value: {}", transaction_value);

    if transaction_value > limit {
        return Err(eyre!("Transaction value is greater than limit price."));
    }

    Ok(transaction_value)
}

/// Calls the buy_shares function on the friend.tech contract.
// TODO: add amount to snipe function
pub async fn send_snipe_transaction(contract: Contract, address: Address, value: U256) -> Result<TransactionReceipt> {
    log::info!("Sending transaction...");

    let transaction = contract.buy_shares(address, U256::from(1))
        .gas(150000)
        .value(value)
        .send()
        .await?
        .await?
        .unwrap();

    Ok(transaction)
}