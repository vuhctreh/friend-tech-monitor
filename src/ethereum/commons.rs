use std::sync::Arc;
use ethers::core::types::{Address};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{Http, Middleware, Wallet};
use ethers::providers::Provider;
use ethers::signers::{LocalWallet, Signer};
use eyre::Result;
use crate::ethereum::contract::FriendtechSharesV1;

pub type Contract = FriendtechSharesV1<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
pub type SignerWallet = Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

#[derive(Clone)]
pub struct WalletCommons {
    pub(crate) provider: Provider<Http>,
    pub(crate) signer: SignerWallet,
    pub(crate) contract: Contract,
    pub(crate) wallet_address: Address,
}

impl WalletCommons {
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
        let contract = FriendtechSharesV1::new("0xCF205808Ed36593aa40a44F10c7f7C2F67d4A4d4"
                                             .parse::<Address>()?, signer.clone());

        Ok(Self {
            provider,
            signer,
            contract,
            wallet_address,
        })
    }
}