use std::sync::Arc;
use ethers::core::types::{Address};
use ethers::contract::abigen;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{Http, Middleware, Wallet};
use ethers::providers::Provider;
use ethers::signers::{LocalWallet, Signer};
use eyre::Result;

pub type Contract = FriendTechV1<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
pub type SignerWallet = Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

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

#[derive(Clone)]
pub struct WalletConfig {
    pub(crate) provider: Provider<Http>,
    pub(crate) signer: SignerWallet,
    pub(crate) contract: Contract,
    pub(crate) wallet_address: Address,
}

impl WalletConfig {
    pub async fn new() -> Result<Self> {
        let wallet_address = std::env::var("WALLET_ADDRESS")?.parse::<Address>()?;

        let provider = Provider::<Http>::try_from(
            std::env::var("RPC_URL")?
        )?;

        let signer = Arc::new({

            let chain_id = provider.get_chainid().await?;

            let wallet = std::env::var("PRIVATE_KEY")?
                .parse::<LocalWallet>()?
                .with_chain_id(chain_id.as_u64());

            SignerMiddleware::new(provider.clone(), wallet)
        });

        // Contract on Goerli
        // let contract = FriendTechV1::new("0xEeaA6B7290F35D588072272E75f1D5eA57827f4f"
        //                                      .parse::<Address>()
        //                                      .expect("ERROR: Could not parse contract."), signer.clone());


        // Contract on Base
        let contract = FriendTechV1::new("0xCF205808Ed36593aa40a44F10c7f7C2F67d4A4d4"
                                             .parse::<Address>()?, signer.clone());

        Ok(Self {
            provider,
            signer,
            contract,
            wallet_address,
        })
    }
}