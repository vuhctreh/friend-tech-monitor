use reqwest::{Client};
use crate::kosetto_api::types::{KosettoResponse, User};
use reqwest::StatusCode;

pub async fn search_user(client: &Client, user: &String) -> Result<KosettoResponse, StatusCode> {

    let url: String = format!("{}{}", std::env::var("KOSETTO_URL").unwrap(), user);

    let resp = client.get(url)
        .send()
        .await
        .expect("ERROR: Failed to get user from Kosetto API.");

    // TODO: Handle response error properly
    match resp.status() {
        StatusCode::OK => Ok(resp.json::<KosettoResponse>().await.expect("ERROR: Failed to parse Kosetto response.")),
        StatusCode::NOT_FOUND => Err(StatusCode::NOT_FOUND),
        _ => Err(resp.status()),
    }
}

pub async fn find_user_in_search(user_info: &KosettoResponse, monitor_target: String) -> Option<&User> {
    for user in user_info.users.iter() {
        if user.twitter_username == monitor_target.clone() {
            log::info!("Found user {}.", monitor_target);
            return Some(user);
        } else {
            log::info!("{} did not match monitor target.", user.twitter_username.clone());
        }
    }

    None
}