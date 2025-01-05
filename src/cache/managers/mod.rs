pub(crate) mod client;
pub(crate) mod channel;
pub(crate) mod guild;

pub(crate) use {
    channel::ChannelManager,
    guild::GuildManager,
    client::ClientManager,
};