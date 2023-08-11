use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Webhook {
    pub content: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub embeds: Option<Vec<Embed>>,
}

impl Webhook {
    pub fn new(content: String) -> Self {
        Self {
            content,
            username: None,
            avatar_url: None,
            embeds: None,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content.to_string();
    }

    pub fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }

    pub fn set_avatar_url(&mut self, avatar_url: String) {
        self.avatar_url = Some(avatar_url);
    }

    pub fn set_embeds(&mut self, embeds: Vec<Embed>) {
        self.embeds = Some(embeds);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Embed {
    title: Option<String>,
    #[serde(rename="type")]
    embed_type: String,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<u32>,
    footer: Option<String>, // TODO: make footer object
    image: Option<String>, // TODO: make image object
    thumbnail: Option<String>, // TODO: Make thumbnail object
    video: Option<String>, // TODO: Make video object
    provider: Option<String>, // TODO: Make provider object
    author: Option<String>, // TODO: Make author object
    fields: Option<Vec<String>>, // TODO: Make field object
}