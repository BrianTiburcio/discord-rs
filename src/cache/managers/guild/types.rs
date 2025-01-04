use serde::{Deserialize, Deserializer};

use crate::cache::CacheManager;
use crate::models::guild::Guild;

#[derive(Debug)]
pub struct GuildManager {
    pub cache: CacheManager<Guild>
}

impl<'de> Deserialize<'de> for GuildManager {
    fn deserialize<D>(_deserializer: D) -> Result<GuildManager, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Create and return the GuildManager instance
        let guild_manager = GuildManager {
            cache: CacheManager::<Guild>::new()
        };

        Ok(guild_manager)
    }
}