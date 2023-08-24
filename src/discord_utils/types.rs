//! Types representing discord Webhook embeds.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Webhook {
    pub content: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub embeds: Option<Vec<Embed>>,
}

impl Webhook {
    // Create empty Webhook struct.
    pub fn new() -> Self {
        Self {
            content: None,
            username: None,
            avatar_url: None,
            embeds: None,
        }
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
    color: u32,
    footer: Footer,
    author: Option<Author>,
}

impl Embed {
    /// Create default Embed struct.
    pub fn new() -> Self {
        Self {
            title: None,
            embed_type: "rich".to_owned(),
            description: None,
            url: None,
            color: 5814783,
            footer: Footer::new(),
            author: None,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn set_author(&mut self, author: Author) {
        self.author = Some(author);
    }
}

#[derive(Serialize, Deserialize)]
struct Footer {
    text: String
}

impl Footer {
    /// Create default Footer struct.
    pub fn new() -> Self {
        Self {
            text: "#RRN".to_string()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    name: String,
    icon_url: String,
}

impl Author {
    /// Create default Author struct.
    pub fn new(name: &String, icon_url: &String) -> Self {
        let name = name.to_string();
        let icon_url = icon_url.to_string();

        Self {
            name,
            icon_url
        }
    }
}