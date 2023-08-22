use reqwest::{Client, Response};
use crate::kosetto_api::types::{KosettoResponse, User};
use eyre::Result;

pub async fn search_user(client: &Client, user: &String, token: String) -> Result<Response> {

    log::info!("Searching for user: {}", user);

    let url: String = format!("{}{}", std::env::var("KOSETTO_URL").unwrap(), user);

    let resp = client.get(url)
        .header("authorization", token)
        .send()
        .await?;

    log::info!("Got user info.");

    Ok(resp)
}

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