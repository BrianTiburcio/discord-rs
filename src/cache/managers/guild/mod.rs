#![allow(dead_code)]
use crate::http::get;
use crate::cache::CacheManager;
use crate::models::guild::Guild;

pub mod types;
pub use types::*;

impl Default for GuildManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GuildManager {
    pub fn new() -> Self {
        Self {
            cache: CacheManager::<Guild>::new(),
        }
    }

    pub fn set(&mut self, guild: Guild) {
        self.cache.set(guild.id.to_owned(), guild);
    }
    
    pub fn get(&self, ids: &[&str]) -> Vec<Guild> {
        let mut collection = Vec::<Guild>::new();

        for id in ids.iter() {
            if let Some(guild) = self.cache.get(id) {
                collection.push(guild.to_owned());
            }
        }

        collection
    }

    // TODO: Create guild
    pub fn fetch(&mut self, ids: &[&str]) -> Vec<Guild> {
        let mut collection = Vec::<Guild>::new();

        for id in ids.iter() {
            if let Some(guild) = self.cache.get(id) {
                collection.push(guild.to_owned());
                continue;
            }

            let guild = _fetch(id);
            collection.push(guild.to_owned());
            self.cache.set(id.to_string(), guild);
        }

        collection
    }
}

fn _fetch(id: &str) -> Guild {
    let response = get(&format!("/guilds/{id}")).unwrap();
    let response = response.text().unwrap();
    serde_json::from_str(&response).unwrap()
}