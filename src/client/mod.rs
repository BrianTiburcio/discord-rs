pub mod event_handler;
pub mod types;

pub use event_handler::*;
pub use types::*;

use std::{
    thread,
    sync::{
        mpsc,
        Arc
    }
};

use crate::{
    models::timestamp::Timestamp,
    cache::managers::{
        ChannelManager,
        GuildManager
    },
    util::env::{
        _set_request_urls,
        _set_client_token,
    },
    util::threadpool::ThreadPool,
    gateway::{
        Gateway,
        enums::{
            GatewayIntents,
            ExternalDispatchEvent,
            ExternalDispatchEventData
        }
    }
};

// The gateway version of Discord's API to use
// https://discord.com/developers/docs/reference#api-versioning-api-versions
const API_VERSION: u8 = 10;

impl Client {
    pub fn new(event_handler: Box<dyn EventHandler>, token: &str, intents: &[GatewayIntents]) -> Result<Client, &'static str> {
        // This creates environment variables for
        // the client token and the API URL which
        // is simpler than passing those values around
        _set_client_token(token);
        _set_request_urls(&API_VERSION);

        // Collapse the intents into a bitfield
        let intents = intents
            .iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            });

        // Create the client instance
        let mut client = Self {
            intents,
            channels: ChannelManager::new(),
            guilds: GuildManager::new(),
            ready_at: Timestamp::now(),
            threadpool: ThreadPool::new(4),
            token: token.to_string(),
            gateway_thread: None,
        };

        // Connect to the gateway socket
        client.connect(intents, event_handler)?;

        // Return the client instance and the event receiver
        Ok(client)
    }

    pub fn disconnect(&mut self) {
        // TODO: close the gateway and events thread if they exist
        todo!();
    }

    /// Connects to Discord's gateway API and begins
    /// receiving and sending data
    fn connect(&mut self, intents: u64, event_handler: Box<dyn EventHandler>) -> Result<(), &'static str> {
        // Create the sender and the receiver channels for the event handler
        let (tx, rx) = mpsc::channel();
        let intents = Arc::new(intents);
        
        // Handle the incoming events as well as heartbeating on a separate thread
        let gateway_thread = thread::spawn({
            let sender = tx.clone();
            let gateway_intents = Arc::clone(&intents);
            move || {
                Gateway::new(&sender, *gateway_intents);
            }
        });

        // Update the client instance with the threads
        self.gateway_thread = Some(gateway_thread);

        // Handle the incoming events on a separate thread
        // These are the events the user should be able to handle
        // but we need to convert the data into usable format
        // and then send it to the user's event handler
        while let Ok((event, data)) = rx.recv() {
            println!("Received event: {:?}\nData: {:?}", event, data);

            match (event, data) {
                (ExternalDispatchEvent::MessageCreate, ExternalDispatchEventData::Message(message)) => {
                    event_handler.message_create(self, message);
                },
                (ExternalDispatchEvent::ChannelCreate, ExternalDispatchEventData::Channel(channel)) => {
                    event_handler.channel_create(self, channel);
                },
                (ExternalDispatchEvent::GuildCreate, ExternalDispatchEventData::Guild(guild)) => {
                    event_handler.guild_create(self, guild);
                },
                _ => {}
            }
        }

        Ok(())
    }
}