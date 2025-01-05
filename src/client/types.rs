use std::thread::JoinHandle;

use crate::{
    models::timestamp::Timestamp,
    util::threadpool::ThreadPool,
    cache::managers::{
        ChannelManager,
        //ClientManager,
        GuildManager
    },
};

pub struct Client {
    pub guilds: GuildManager,
    pub channels: ChannelManager,
    pub intents: u64,
    // TODO: pub user: User,
    //pub(crate) handler: Box<dyn Handler>,
    pub(crate) ready_at: Timestamp,
    pub(crate) token: String,
    pub(crate) threadpool: ThreadPool,
    pub(crate) gateway_thread: Option<JoinHandle<()>>
}