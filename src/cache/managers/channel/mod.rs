use crate::cache::CacheManager;
use crate::models::{
    channel::Channel,
    snowflake::Snowflake
};
//use crate::util::rest::get;

mod types;
pub use types::*;

#[derive(Debug)]
pub enum ChannelError {
    NotFound
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelManager {
    pub fn new() -> Self {
        Self { cache: CacheManager::<Box<Channel>>::new() }
    }

    pub fn set_by_id(&mut self, id: &Snowflake) {
        Self::_patch(self, id);
    }

    pub fn set(&mut self, channel: Channel) {
        Self::_patch(self, &channel.id);
    }

    pub fn fetch_by_id(&mut self, id: &Snowflake) -> Result<Box<Channel>, ChannelError> {
        let channels = Self::fetch(self, &[id]);
        if channels.len() == 1 {
            return Ok(channels[0].clone());
        }
    
        Err(ChannelError::NotFound)
    }

    pub fn fetch(&mut self, ids: &[&Snowflake]) -> Vec<Box<Channel>> {
        let mut collection = Vec::<Box<Channel>>::new();

        for id in ids.iter() {
            if let Some(channel) = self.cache.get(&id.to_string()) {
                collection.push(channel.to_owned());
                continue;
            }

            let boxed_channel = Box::new(
                Channel::new(&Snowflake::from(*id)).unwrap()
            );
            
            collection.push(boxed_channel.clone());
            self.cache.set(id.to_string(), boxed_channel);
        }

        collection
    }

    fn _patch(&mut self, id: &Snowflake) -> Box<Channel> {
        let boxed_channel = Box::new(
            Channel::new(&Snowflake::from(id)).unwrap()
        );
        
        self.cache.set(id.to_string(), boxed_channel.to_owned());
        boxed_channel
    }
}