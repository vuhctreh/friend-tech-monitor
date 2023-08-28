use std::any::TypeId;
use std::ops::Add;
use std::str::FromStr;
use ethers::abi::AbiDecode;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, Block, BlockNumber, Transaction, U256, U64};
use eyre::Result;
use crate::ethereum::contract::FriendtechSharesV1Calls;

const ADDRESS: &str = "0xcf205808ed36593aa40a44f10c7f7c2f67d4a4d4";

pub async fn get_previous_block_txs(provider: &Provider<Http>) -> Result<Option<Vec<Transaction>>> {
    let block_number: BlockNumber = BlockNumber::from(provider.get_block_number().await?);

    let transactions: Option<Block<Transaction>> = provider.get_block_with_txs(block_number).await?;

    match transactions {
        Some(block) => Ok(Some(block.transactions)),
        None => Ok(None)
    }
}

pub fn filter_signup_txs(txs: Vec<Transaction>) -> Result<Vec<FriendtechSharesV1Calls>> {
    let friend_tech_address: Address = Address::from_str(ADDRESS)?;

    let mut filtered_txs: Vec<FriendtechSharesV1Calls> = vec![];

    for tx in txs {
        match tx.to {
            Some(address) => {
                if address == friend_tech_address && tx.value == U256::zero() {
                    let data = FriendtechSharesV1Calls::decode(&tx.input)?;

                    match data {
                        FriendtechSharesV1Calls::BuyShares(..) => {
                            filtered_txs.push(data);
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(filtered_txs)
}

#[tokio::test]
async fn test_get_previous_block_txs() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://developer-access-mainnet.base.org")?;
    let previous_block_txs = get_previous_block_txs(&provider).await?;

    match previous_block_txs {
        Some(txs) => {
            println!("{:?}", filter_signup_txs(txs)?);
        },
        None => {}
    }

    Ok(())
}