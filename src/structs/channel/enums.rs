use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/topics/permissions#permissions
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PermissionOverwriteType {
    CreateInstantInvite,
    KickMembers,
    BanMembers,
    Administrator,
    ManageChannels,
    ManageGuild,
    AddReactions,
    ViewAuditLogs,
    PrioritySpeaker,
    Stream,
    ViewChannel,
    SendMessages,
    SendTTSMessages,
    ManageMessages,
    EmbedLinks,
    AttachFiles,
    ReadMessageHistory,
    MentionEveryone,
    UseExternalEmojis,
    ViewGuildInsights,
    Connect,
    Speak,
    MuteMembers,
    DeafenMembers,
    MoveMembers,
    UseVAD,
    ChangeNickname,
    ManageNicknames,
    ManageRoles,
    ManageWebhooks,
    ManageGuildExpressions,
    UseApplicationCommands,
    RequestToSpeak,
    ManageEvents,
    ManageThreads,
    CreatePublicThreads,
    CreatePrivateThreads,
    UseExternalStickers,
    SendMessagesInThreads,
    UseEmbeddedActivities,
    ModerateMembers,
    ViewCreatorMonetizationAnalytics,
    UseSoundBoard,
    UseExternalSounds,
    SendVoiceMessages
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChannelType {
    GuildText,
    DM,
    GuildVoice,
    GroupDM,
    GuildCategory,
    GuildAnnouncement,
    AnnouncementThread,
    PublicThread,
    PrivateThread,
    GuildStageVoice,
    GuildDirectory,
    GuildForum
}