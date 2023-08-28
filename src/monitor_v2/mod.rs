//! Some constraints: We are looking for first share purchases on friend.tech (a.k.a when a
//! user first signs up). From the contract, we know that only the user themselves may purchase
//! their own first share, and that at a price of 0. Thus, we can assume that every successful
//! buyShares contract call with a value of 0 is a first share purchase. Since Base is built on
//! the OP stack, there is no mempool for pending transactions. What we must do instead is
//! go through each block and look for calls to the friend.tech - parsing transactions accordingly.
//! The problem: there is a block every 2∼ seconds, and, although the ETH JSON RPC provides us
//! the eth_getTransactionReceipts method, it only lists transaction hashes. Thus, we have to
//! call the JSON RPC endpoint for each transaction receipt and parse each one individually.
//! This is slow and may not fit into the 2s time frame if there are too many transactions
//! (each rpc call is ∼120ms). Alternatively, we could use the alchemy_getTransactionReceipts method
//! which shows us data for each transaction (including logs). The problem here is that we don't get
//! call data from this method. Luckily, friend.tech only has 2 events - Trade (what we're interested
//! in) and OwnershipTransferred (which is unlikely to be emitted).
//! The final option would be to call eth_getTransactionReceipts and simply parse each transaction
//! in parallel.
//!
//! **Note: the alchemy_getTransactionReceipts method uses 250CU (computing units) per call.**
//! **This means, if we call it every 2 seconds, our use case goes beyond the free tier.**
// TODO: Rewrite this ^

pub mod monitor_v2;