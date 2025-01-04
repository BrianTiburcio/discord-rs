use crate::{
    http,
    models::{
        message_payload::MessagePayload,
        message::Message,
        permissions::Permissions,
        snowflake::Snowflake
    }
};

pub mod types;
pub use types::*;

pub mod enums;
pub use enums::*;

mod deserializers;
use deserializers::*;

impl Channel {
    pub fn new(channel_id: &Snowflake) -> Result<Self, &'static str> {
        // Directly call _fetch without introducing recursion
        Self::_fetch(channel_id)
    }

    /// Send a [MessagePayload] to this channel
    /// Returns the message that was sent if successful
    /// # Errors
    /// Returns an error if the message was not sent successfully
    pub fn send(&self, payload: MessagePayload) -> Result<Message, &'static str> {
        let payload = serde_json::to_string(&payload).unwrap();
        
        let res = http::post(&format!("/channels/{}/messages", &self.id), &payload)
            .expect("Failed to send message to channel");

        if !res.status().is_success() {
            return Err("Message was not sent successfully");
        }

        let message = res.json::<Message>().expect("Failed to deserialize message");

        println!("Sent message => {}. Response => {:?}. ", payload, message);
        Ok(message)
    }

    /// Fetch a channel from the API
    pub(crate) fn _fetch(channel_id: &Snowflake) -> Result<Channel, &'static str> {
        println!("Fetching channel: {}", channel_id);
        let request = http::get(&format!("/channels/{}", channel_id))
            .expect("Request failed to send");

        println!("Request sent: {:?}", request);
    
        let json_string = request.text()
            .expect("Failed to fetch channel from API");

        println!("Fetched channel: {}", json_string);
    
        // Directly deserialize into a Channel without calling Channel::new
        let channel: Channel = serde_json::from_str(&json_string)
            .expect("Failed to deserialize the channel object");
    
        println!("Deserialized channel: {:?}", channel);

        Ok(channel)
    }
}

impl PermissionOverwrite {
    pub fn to_bit(&self, bits: &[Permissions]) -> u64 {
        bits.iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            })
    }
}

