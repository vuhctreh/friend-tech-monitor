use ethers::types::{Address, U256};
use ethers::utils::parse_ether;
use eyre::{eyre, Result};
use crate::ethereum::config::{Contract};



pub async fn prepare_snipe(contract: Contract, address: Address) -> Result<U256> {
    let limit: U256 = parse_ether(std::env::var("LIMIT_PRICE")?)?;

    // TODO: amount hard coded to 1 for now, rework json to include limit
    let mut transaction_value: U256 = contract.get_buy_price_after_fee(address, 1).await?;

    log::info!("Limit price: {}", limit);
    log::info!("Projected transaction value: {}", transaction_value);

    if transaction_value > limit {
        return Err(eyre!("Transaction value is greater than limit price."));
    }

    Ok(transaction_value)
}