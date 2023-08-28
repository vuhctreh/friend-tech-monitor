//! Types linked to the Kosetto API.
use serde::Deserialize;

pub trait User {
    fn get_username(&self) -> String;
    fn get_name(&self) -> String;
    fn get_pfp_url(&self) -> String;
    fn get_address(&self) -> String;

}

#[derive(Deserialize, Clone)]
pub struct KosettoUserSearchResponse {
    pub users: Vec<SearchedUser>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchedUser {
    pub address: String,
    pub twitter_username: String,
    pub twitter_name: String,
    pub twitter_pfp_url: String,
    pub twitter_user_id: String,
}

impl User for SearchedUser {
    fn get_username(&self) -> String {
        self.twitter_username.clone()
    }

    fn get_name(&self) -> String {
        self.twitter_name.clone()
    }

    fn get_pfp_url(&self) -> String {
        self.twitter_pfp_url.clone()
    }

    fn get_address(&self) -> String {
        self.address.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExactUser {
    pub id: u64,
    pub address: String,
    pub twitter_username: String,
    pub twitter_name: String,
    pub twitter_pfp_url: String,
    pub twitter_user_id: String,
    pub last_online: String,
    pub holder_count: u64,
    pub holding_count: u64,
    pub share_supply: u64,
    pub display_price: String,
    pub lifetime_fees_collected_in_wei: String,
}

impl User for ExactUser {
    fn get_username(&self) -> String {
        self.twitter_username.clone()
    }

    fn get_name(&self) -> String {
        self.twitter_name.clone()
    }

    fn get_pfp_url(&self) -> String {
        self.twitter_pfp_url.clone()
    }

    fn get_address(&self) -> String {
        self.address.clone()
    }
}