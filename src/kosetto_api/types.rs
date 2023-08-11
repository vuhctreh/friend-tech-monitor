use serde::Deserialize;

#[derive(Deserialize)]
pub struct KosettoResponse {
    pub users: Vec<User>,
}

#[derive(Deserialize)]
pub struct User {
    pub address: String,
    #[serde(rename="twitterUsername")]
    twitter_username: String,
    #[serde(rename="twitterName")]
    twitter_name: String,
    #[serde(rename="twitterPfpUrl")]
    twitter_pfp_url: String,
    #[serde(rename="twitterUserId")]
    twitter_user_id: String,
}