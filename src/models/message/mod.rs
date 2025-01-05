//use crate::client::ClientCache;
use crate::models::{
    snowflake::Snowflake,
    message_payload::{
        MessagePayload,
        MessageReferenceType,
        MessageReference
    },
    channel::Channel
};

pub mod enums;
pub mod types;

pub use enums::*;
pub use types::*;

impl Message {
    // pub fn from(json: Value, cache: Arc<Mutex<ClientCache>>) -> Result<Self, &'static str> {
    //     let mut message: Self = serde_json::from_value(json).expect("Failed to parse message from JSON");
        
    //     if let Some(channel_id) = &message.channel_id {
    //         let mut cache = cache.lock().await;
    //         let channel = cache.channels.fetch_by_id(channel_id).await.unwrap();
    //         message.channel = Some(channel);
    //     }

    //     Ok(message)
    // }

    // /// Sends a text message to the channel
    // pub fn reply_content(&self, content: &str) -> Result<(), &'static str> {
    //     let mut payload = MessagePayload::new();
    //     payload.content = Some(content.to_string());
    //     Self::reply(self, payload)
    // }

    pub fn get_channel(&mut self) -> Option<&Channel> {
        // This message doesnt belong to a channel
        // therefore there is no channel to return
        if self.channel_id.is_none() {
            return None;
        }

        // At this point, we know there is a channel_id
        // but no channel object... yet
        if self.channel.is_none() {
            // Fetch the channel from the API
            let channel = Channel::_fetch(self.channel_id.as_ref().unwrap());
            // Update the message with the channel
            self.channel = Some(channel.unwrap());
        }

        self.channel.as_ref()
    }

    /// Sends payloads which may include text, embeds, tts and more to the channel
    pub fn reply(&mut self, mut payload: MessagePayload) -> Result<Message, &'static str> {
        // First, gather all immutable references needed
        let message_id = self.id.clone();

        let channel = self.get_channel();
        if channel.is_none() {
            return Err("Failed to get channel");
        }

        let channel = channel.unwrap();
        let channel_id = channel.id.clone();
        let guild_id = channel.guild_id.clone();

        // Modify the payload
        payload.message_reference = Some(MessageReference {
            reference_type: MessageReferenceType::Default,
            message_id: Some(message_id),
            channel_id: Some(channel_id),
            guild_id: guild_id,
            fail_if_not_exists: Some(false),
        });

        // Finally, send the message
        let message = channel.send(payload)?;
        Ok(message)
    }

    /// Whether or not the message can be deleted
    pub fn is_deletable(&self) -> bool {
        matches!(self.message_type, MessageType::Default
            | MessageType::ChannelPinnedMessage
            | MessageType::UserJoin
            | MessageType::GuildBoost
            | MessageType::GuildBoostTier1
            | MessageType::GuildBoostTier2
            | MessageType::GuildBoostTier3
            | MessageType::ChannelFollowAdd
            | MessageType::GuildDiscoveryGracePeriodInitialWarning
            | MessageType::GuildDiscoveryGracePeriodFinalWarning
            | MessageType::Reply
            | MessageType::ChatInputCommand
            | MessageType::GuildInviteReminder
            | MessageType::ContextMenuCommand
            | MessageType::AutoModerationAction
            | MessageType::RoleSubscriptionPurchase
            | MessageType::InteractionPremiumUpsell
            | MessageType::StageStart
            | MessageType::StageEnd
            | MessageType::StageSpeaker
            | MessageType::StageTopic
            | MessageType::GuildApplicationPremiumSubscription
        )
    }

    /// Whether or not the message is
    pub fn is_channel_message(&self) -> bool {
        todo!();
    }
}