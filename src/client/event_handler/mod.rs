use crate::{
    client::Client,
    models::message::Message,
    models::channel::Channel,
    models::guild::Guild
};

pub trait EventHandler {
    fn on_ready(&self, _client: &mut Client) {}
    
    // Message Events
    fn message_create(&self, _client: &mut Client, _message: Message) {}
    fn message_update(&self, _client: &mut Client, _message: Message) {}
    fn message_delete(&self, _client: &mut Client, _message: Message) {}
    fn message_delete_bulk(&self, _client: &mut Client, _messages: Vec<Message>) {}
    
    // Channel Events
    fn channel_create(&self, _client: &mut Client, _channel: Channel) {}
    fn channel_update(&self, _client: &mut Client, _channel: Channel) {}
    fn channel_delete(&self, _client: &mut Client, _channel: Channel) {}
    
    // Guild Events
    fn guild_create(&self, _client: &mut Client, _guild: Guild) {}
    fn guild_update(&self, _client: &mut Client, _guild: Guild) {}
    fn guild_delete(&self, _client: &mut Client, _guild: Guild) {}
}