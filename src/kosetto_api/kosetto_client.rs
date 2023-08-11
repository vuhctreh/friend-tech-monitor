use reqwest::Client;
use crate::kosetto_response::KosettoResponse;

pub(crate) async fn get_user(client: &Client, user: String) -> Result<KosettoResponse, reqwest::Error> {

    let resp = client.get(&format!("https://prod-api.kosetto.com/search/users?username={}", user))
        .send()
        .await?
        .json::<KosettoResponse>()
        .await?;

    Ok(resp)
}