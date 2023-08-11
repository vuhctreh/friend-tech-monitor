use serde::Deserialize;

#[derive(Deserialize)]
pub struct KosettoResponse {
    pub users: Vec<User>,
}

#[derive(Deserialize)]
pub struct User {
    pub address: String,
    #[serde(rename="twitterUsername")]
    pub twitter_username: String,
    #[serde(rename="twitterName")]
    pub twitter_name: String,
    #[serde(rename="twitterPfpUrl")]
    pub twitter_pfp_url: String,
    #[serde(rename="twitterUserId")]
    pub twitter_user_id: String,
}