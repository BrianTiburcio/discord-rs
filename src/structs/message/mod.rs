//use crate::client::ClientCache;
use crate::structs::channel::Channel;

pub mod enums;
pub use enums::*;

pub mod structs;
pub use structs::*;

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

    pub fn get_channel(&mut self) -> &Channel {
        todo!()

        // if self.channel.is_some() {
        //     return Ok(*self.channel)
        // }

        // if let Some(channel) = &self.channel {
        //     return Ok(channel);
        // }

        // if let Some(channel_id) = self.channel_id.take() {
        //     let channel = Channel::new(&channel_id)?;
        //     self.channel = Some(channel);
            
        //     if let Some(channel) = &self.channel {
        //         Ok(channel)
        //     } else {
        //         Err("Failed to obtain channel")
        //     }
        // } else {
        //     Err("No channel to get")
        // }
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

    // Sends payloads which may include text, embeds, tts and more to the channel
    // pub async fn reply(&self, payload: MessagePayload) -> Result<(), &'static str> {
    //     if self.channel.is_none() {
    //         return Err("No channel available to reply");
    //     }

    //     let channel = self.channel.as_ref().unwrap();
    //     let _ = channel.send(payload).await;

    //     Ok(())
    // }
}