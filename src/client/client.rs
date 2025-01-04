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
    gateway::Gateway
};

use super::{
    Handler,
    Client,
    enums::{
        GatewayIntents,
        ExternalDispatchEvent,
        ExternalDispatchEventData
    }
};

// The gateway version of Discord's API to use
// https://discord.com/developers/docs/reference#api-versioning-api-versions
const API_VERSION: u8 = 10;

impl Client {
    pub fn new(event_handler: Box<dyn Handler>, token: &str, intents: &[GatewayIntents]) -> Self {
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

        Self {
            channels: ChannelManager::new(),
            guilds: GuildManager::new(),
            ready_at: Timestamp::now(),
            threadpool: ThreadPool::new(4),
            token: token.to_string(),
            handler: event_handler,
            intents
        }
    }

    /// Connects to Discord's gateway API and begins
    /// receiving and sending data
    pub fn connect(&mut self) -> Result<mpsc::Receiver<(ExternalDispatchEvent, Option<ExternalDispatchEventData>)>, &'static str> {
        // Create the sender and the receiver channels for the event handler
        let (tx, rx) = mpsc::channel();
        let intents = Arc::new(self.intents);

        // Handle the incoming events as well as heartbeating on a separate thread
        let _gateway_thread = thread::spawn(move || Gateway::new(&tx, *intents.clone()));

        // Ideally here we'd yield the receiver
        // but yielding is not yet stable in rust
        
        // yield Ok(rx);

        // Join the execution of the event loop to the main
        // thread so that the main thread doesnt exit until
        // the event handler loop is done, which ideally
        // shouldnt happen as long as the bot is active
        
        // P.S. Will block the main thread.
        // Wait until yield stabilizes to implement
        
        // let _ = event_handler_thread.join();

        Ok(rx)
    }
}