use std::sync::Arc;
use ethers::core::types::{Address};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{Http, Middleware, Wallet};
use ethers::providers::Provider;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::U256;
use eyre::Result;
use reqwest::Client;
use crate::ethereum::contract::FriendtechSharesV1;
use crate::sniper::sniper_contract::BruhTech;

pub type Contract = BruhTech<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
pub type SignerWallet = Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

#[derive(Clone)]
pub struct ApplicationCommons {
    pub(crate) provider: Provider<Http>,
    pub(crate) signer: SignerWallet,
    pub(crate) contract: Contract,
    pub(crate) client: Arc<Client>,
}

impl ApplicationCommons {
    pub fn new() -> Result<Self> {

        let client: Arc<Client> = Arc::new(Client::new());

        let provider: Provider<Http> = Provider::<Http>::try_from(
            std::env::var("RPC_URL")?
        )?;

        let signer: SignerWallet = Arc::new({

            let chain_id: U256 = U256::from(8453);

            let wallet: Wallet<SigningKey> = std::env::var("PRIVATE_KEY")?
                .parse::<LocalWallet>()?
                .with_chain_id(chain_id.as_u64());

            SignerMiddleware::new(provider.clone(), wallet)
        });

        let contract: Contract = BruhTech::new("0x12EB1263F1755CF29b6c4D5Fb3EEa908c1769105"
                                             .parse::<Address>()?, signer.clone());

        Ok(Self {
            provider,
            signer,
            contract,
            client
        })
    }
}