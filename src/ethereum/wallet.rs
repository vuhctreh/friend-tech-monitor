use std::sync::Arc;

use ethers::{
    core::types::{Address, U256},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
};

async fn test() -> Result<(), String> {
    let provider = Arc::new({

        let provider = Provider::<Http>::try_from(
            std::env::var("RPC_URL").expect("RPC not set in .env.")
        ).expect("Invalid RPC URL.");

        let chain_id = provider.get_chainid().await?;

        let wallet = std::env::var("PRIVATE_KEY")
            .unwrap()
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(provider, wallet)
    });

    let balance: U256 = provider.get_balance(Address::zero()).await?;

    println!("{}", balance);

    Ok(())
}