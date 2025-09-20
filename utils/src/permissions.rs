use serenity::all::Permissions;

#[derive(Debug, Clone)]
pub enum PermissionLevel {
    BotMaster,
    Administrator,
    Moderator,
    User,
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
