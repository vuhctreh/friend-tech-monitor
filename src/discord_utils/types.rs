use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Webhook {
    pub(crate) content: String,
    // username: Option<String>,
    // avatar_url: Option<String>,
    // tts: Option<bool>,
    // embeds: Option<Vec<String>>, //TODO make embed object
    // allowed_mentions: Option<String>, // TODO make allowed_option object
    // components: Option<Vec<String>>, //TODO make message object
    // files: Option<String>, // TODO ?
    // payload_json: Option<String>, // TODO ?
    // attachments: Option<Vec<String>>, //TODO make attachment object
    // flags: Option<u64>,
    // thread_name: Option<String>,
}