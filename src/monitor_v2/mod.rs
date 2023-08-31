//! Monitor V1 (now deprecated) was implemented such that it would
//! ping the friend.tech "search" endpoint every n seconds for each
//! "monitored" profile. Although this was reliable, it was fairly
//! slow, prone to rate limiting (although afaik there was none in
//! place) and not parallel (thus longer lists = worse performance).
//!
//! The new monitor calls the Base JSON RPC, grabs and filters
//! looks for "buyShares" calls of value 0. Since Base uses the
//! Optimism stack, the mempool is private; we can therefore not
//! use pending transactions. Instead, we get transactions from each block
//! and filter them as mentioned above, after which we call the friend.tech
//! users/{address} endpoint to get specific details on the user. This
//!
//! Since there are blocks every 2 seconds,
//! we must make sure this process finished within the 2 second window.
//!
//! Most of the performance relies on api response times (RPC, friend.tech).
//! Whereas the only real way to improve RPC performance is by using a better RPC,
//! We can call friend.tech on a separate thread, avoiding blocking the monitor.
//! It is also important to note that there is a delay of ≈1s between confirmed
//! first share purchases and a user signing up. By calling this endpoint,
//! concurrently with the monitor, we can avoid this delay.

pub mod monitor_v2;