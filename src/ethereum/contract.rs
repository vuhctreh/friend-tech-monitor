use std::sync::Arc;
use ethers::core::types::{Address, U256};
use ethers::contract::abigen;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{Http, TransactionReceipt, U64, Wallet};
use ethers::providers::Provider;

abigen!(FriendTechV1, r#"[
        function buyShares(address,uint256) external
        function renounceOwnership() external
        function sellShares(address,uint256) external
        function setFeeDestination(address) external
        function setProtocolFeePercent(uint256) external
        function transferOwnership(address) external
        function getBuyPrice(address,uint256) external view returns ( uint256 )
        function getBuyPriceAfterFee(address,uint256) external view returns ( uint256 )
        function getPrice(uint256,uint256) external pure returns ( uint256 )
        function getSellPrice(address,uint256) external view returns ( uint256 )
        function getSellPriceAfterFee(address,uint256) external view returns ( uint256 )
        function owner() external view returns ( address )
        function protocolFeeDestination() external view returns ( address )
        function protocolFeePercent() external view returns ( uint256 )
        function sharesBalance(address,address) external view returns ( uint256 )
        function sharesSupply(address) external view returns ( uint256 )
        function subjectFeePercent() external view returns ( uint256 )
    ]"#);

pub async fn call_buy_shares(provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, buy_address: Address, amount: U256) -> TransactionReceipt {
    let contract_address: Address = "0xEeaA6B7290F35D588072272E75f1D5eA57827f4f".parse::<Address>().expect("Could not parse contract");

    let contract = FriendTechV1::new(contract_address, provider.clone());

    let transaction_value: U256 = contract.get_buy_price(buy_address.clone(), amount.clone()).await.unwrap();

    println!("Current price of shares: {}", &transaction_value);

    println!("Attempting purchase...");

    let transaction = contract.buy_shares(buy_address.clone(), amount)
        .gas(30000)
        .value(&transaction_value)
        .send()
        .await
        .expect("Failed to buy shares.")
        .await
        .expect("Failed to buy shares.")
        .unwrap();

    println!("Transaction status: {:?}", &transaction.status);

    transaction
}