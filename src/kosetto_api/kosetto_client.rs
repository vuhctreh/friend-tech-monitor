//! Implementations for calling the Kosetto (friend.tech) API.

use std::sync::Arc;
use ethers::types::Address;
use reqwest::{Client, Response};
use eyre::Result;

const URL: &str = "https://prod-api.kosetto.com/users/";

/// Gets a user by their address. This endpoint does not need a token.
pub async fn get_user_by_address(client: Arc<Client>, address: Address) -> Result<Response> {
    log::info!("Getting user info for address: {}", address);

    // Read this for explanation on formatting:
    // https://stackoverflow.com/questions/57350082/to-convert-a-ethereum-typesh256-to-string-in-rust
    let url: String = format!("{}{:#x}", URL, address);

    let resp = client.get(url)
       .send()
       .await?;

    log::info!("Got user info.");

    Ok(resp)
}