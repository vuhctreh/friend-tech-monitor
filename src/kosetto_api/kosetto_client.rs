use reqwest::{Client, Response};
use crate::kosetto_api::types::KosettoResponse;
use reqwest::StatusCode;
use crate::discord_utils::types::Webhook;
use crate::discord_utils::webhook_utils::{post_webhook, prepare_webhook};

pub async fn get_user(client: &Client, user: &String) -> Result<KosettoResponse, StatusCode> {

    let url: String = format!("{}{}", std::env::var("KOSETTO_URL").unwrap(), user);

    let resp = client.get(url)
        .send()
        .await
        .expect("ERROR: Failed to get user from Kosetto API.");

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<KosettoResponse>().await.expect("ERROR: Failed to parse Kosetto response.")),
        StatusCode::NOT_FOUND => Err(StatusCode::NOT_FOUND),
        _ => Err(resp.status()),
    }
}

pub async fn monitor(user_info: &KosettoResponse, monitor_target: String, client: &Client, webhook_url: &String) -> Result<u8, String> {
    for user in user_info.users.iter() {
        if user.twitter_username == monitor_target.clone() {
            let webhook: Webhook = prepare_webhook(user.twitter_username.clone(),
                                                   user.twitter_pfp_url.clone(),
                                                   user.address.clone(),
                                                   user.twitter_name.clone());

            let resp: Result<Response, String> = post_webhook(&client, &webhook_url, &webhook).await;

            return match resp {
                Ok(_) => {
                    println!("LOG: Posted webhook to discord.");
                    println!("LOG: Removed {} from monitor list.", monitor_target);
                    Ok(1)
                }
                Err(_) => {
                    Err("ERROR: Could not post webhook to discord.".to_string())
                }
            }
        } else {
            println!("LOG: {} did not match monitor target.", user.twitter_username.clone());
            continue;
        }
    }
    Ok(2)
}