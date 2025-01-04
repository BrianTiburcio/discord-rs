use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::models::{
    attachment::Attachment,
    embed::Embed,
    snowflake::Snowflake,
    nonce::Nonce
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AllowedMentionsType {
    RoleMentions,
    UserMentions,
    EveryoneMentions
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllowedMentions {
    pub parse: Vec<AllowedMentionsType>,
    /// Max size 100
    pub roles: Vec<String>,
    /// Max size 100
    pub users: Vec<String>,
    pub replied_user: bool
}

// Dont serialize any of the optionals if they are of the None variant
#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct MessagePayload {
    pub content: Option<String>,
    pub nonce: Option<Nonce>,
    pub tts: Option<bool>,
    pub embeds: Option<Vec<Embed>>,
    pub allowed_mentions: Option<AllowedMentions>,
    // TODO: Make MessageReference object
    pub message_reference: Option<MessageReference>,
    // TODO: Make Component object
    pub components: Option<Value>,
    pub sticker_ids: Option<Vec<Snowflake>>,
    // TODO: Make File object
    pub files: Option<Value>,
    pub payload_json: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub flags: Option<usize>

    // These fields are only usable on webhook payloads
    // pub username: Option<String>,
    // pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageReferenceType {
    /// A standard reference used by replies
    Default,
    /// Reference used to point to a message at a point in time
    Forward
}

// https://discord.com/developers/docs/resources/message#message-reference-structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReference {
    #[serde(rename = "type")]
    pub reference_type: MessageReferenceType,
    pub message_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>
}