use serenity::all::Permissions;
use strum_macros::Display;

#[derive(Display, Debug, Clone)]
pub enum BotPermission {
    #[strum(serialize = "bot_master")]
    BotMaster,
    #[strum(serialize = "administrator")]
    Administrator,
    #[strum(serialize = "ban_members")]
    BanMembers,
    #[strum(serialize = "kick_members")]
    KickMembers,
    #[strum(serialize = "mute_members")]
    MuteMembers,
    #[strum(serialize = "deafen_members")]
    DeafenMembers,
    #[strum(serialize = "move_members")]
    MoveMembers,
    #[strum(serialize = "manage_guild")]
    ManageGuild,
    #[strum(serialize = "manage_channels")]
    ManageChannels,
    #[strum(serialize = "manage_roles")]
    ManageRoles,
    #[strum(serialize = "manage_messages")]
    ManageMessages,
    #[strum(serialize = "manage_webhooks")]
    ManageWebhooks,
    #[strum(serialize = "manage_guild_expressions")]
    ManageGuildExpressions,
    #[strum(serialize = "manage_events")]
    ManageEvents,
    #[strum(serialize = "manage_nicknames")]
    ManageNicknames,
    #[strum(serialize = "mention_everyone")]
    MentionEveryone,
}

pub const PERMISSION_PRIORITY: [Permissions; 32] = [
    // 🔒 Superuser
    Permissions::ADMINISTRATOR,
    // 🏗 Guild management
    Permissions::MANAGE_GUILD,
    Permissions::MANAGE_ROLES,
    Permissions::MANAGE_CHANNELS,
    Permissions::MANAGE_WEBHOOKS,
    Permissions::MANAGE_GUILD_EXPRESSIONS,
    Permissions::MANAGE_EVENTS,
    Permissions::MANAGE_THREADS,
    // 👮 Moderation
    Permissions::BAN_MEMBERS,
    Permissions::KICK_MEMBERS,
    Permissions::MODERATE_MEMBERS, // Timeout
    Permissions::MUTE_MEMBERS,
    Permissions::DEAFEN_MEMBERS,
    Permissions::MOVE_MEMBERS,
    Permissions::MANAGE_MESSAGES,
    Permissions::MENTION_EVERYONE,
    Permissions::VIEW_AUDIT_LOG,
    Permissions::MANAGE_NICKNAMES,
    // 📢 Messaging / Voice control
    Permissions::PRIORITY_SPEAKER,
    Permissions::SPEAK,
    Permissions::STREAM,
    Permissions::CONNECT,
    Permissions::SEND_MESSAGES,
    Permissions::SEND_TTS_MESSAGES,
    Permissions::EMBED_LINKS,
    Permissions::ATTACH_FILES,
    Permissions::ADD_REACTIONS,
    Permissions::USE_EXTERNAL_EMOJIS,
    Permissions::USE_EXTERNAL_STICKERS,
    Permissions::USE_APPLICATION_COMMANDS,
    // 👀 Viewing
    Permissions::VIEW_CHANNEL,
    Permissions::READ_MESSAGE_HISTORY,
];
