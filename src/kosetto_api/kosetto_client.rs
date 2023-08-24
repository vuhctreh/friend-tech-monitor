//! Implementations for calling the Kosetto (friend.tech) API.

use reqwest::{Client, Response};
use crate::kosetto_api::types::{KosettoResponse, User};
use eyre::Result;

/// Calls search/users to check if a user has has signed up.
pub async fn search_user(client: &Client, user: &String, token: String) -> Result<Response> {

    log::info!("Searching for user: {}", user);

    let url: String = format!("https://prod-api.kosetto.com/search/users?username={}", user);

    let resp = client.get(url)
        .header("authorization", token)
        .send()
        .await?;

    log::info!("Got user info.");

    Ok(resp)
}

// Finds exact match for a user in search_user response.
pub fn find_user_in_search(user_info: &KosettoResponse, monitor_target: &String) -> Option<User> {
    for user in user_info.users.iter() {
        if user.twitter_username == monitor_target.clone() {
            log::info!("Found user {}.", monitor_target);
            return Some(user.clone());
        } else {
            log::info!("{} did not match monitor target.", user.twitter_username.clone());
        }
    }

    None
}