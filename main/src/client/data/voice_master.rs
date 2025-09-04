use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use serenity::{
    all::{
        ChannelId, CreateChannel, GuildId, PermissionOverwrite, PermissionOverwriteType,
        Permissions, UserId,
    },
    prelude::TypeMapKey,
};
use tokio::sync::RwLock;
use utils::{BotHash, BotStringParser, UserConfigHash};
pub struct VoiceHub;
impl TypeMapKey for VoiceHub {
    type Value = Arc<RwLock<BotHash<GuildId, VoiceMasterConfig>>>;
}

pub struct UserVoiceConfigRepo;
impl TypeMapKey for UserVoiceConfigRepo {
    type Value = Arc<RwLock<UserConfigHash<VoiceConfig>>>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VoiceMasterConfig {
    master: Vec<MasterVoiceChannel>,
    active: Vec<ActiveVoiceChannel>,
    config: Option<VoiceConfig>,
    parent_id: Option<ChannelId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveVoiceChannel {
    pub id: ChannelId,
    pub owner: UserId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterVoiceChannel {
    pub id: ChannelId,
    pub category: Option<u64>,
}

impl MasterVoiceChannel {
    pub fn new(id: ChannelId, category: Option<u64>) -> Self {
        Self { id, category }
    }
}

impl VoiceMasterConfig {
    pub fn new(master: Vec<MasterVoiceChannel>) -> Self {
        Self {
            master,
            active: Vec::new(),
            config: None,
            parent_id: None,
        }
    }

    pub fn is_master(&self, channel: &ChannelId) -> bool {
        self.master.iter().any(|c| c.id == *channel)
    }

    pub fn add_active_channel(&mut self, channel: ChannelId, owner: UserId) -> ActiveVoiceChannel {
        let active_channel = ActiveVoiceChannel { id: channel, owner };
        self.active.push(active_channel.clone());
        active_channel
    }

    pub fn remove_active_channel(&mut self, channel: &ChannelId) -> Option<ActiveVoiceChannel> {
        match self.active.iter().position(|c| c.id == *channel) {
            Some(pos) => Some(self.active.remove(pos)),
            None => None,
        }
    }

    pub fn is_active(&self, channel: &ChannelId) -> bool {
        self.active.iter().any(|c| c.id == *channel)
    }
    pub fn is_owner(&self, channel: &ChannelId, user: &UserId) -> bool {
        self.active
            .iter()
            .any(|c| c.id == *channel && c.owner == *user)
    }

    pub fn set_config(&mut self, config: VoiceConfig) {
        self.config = Some(config);
    }

    pub fn set_parent_id(&mut self, parent_id: ChannelId) {
        self.parent_id = Some(parent_id);
    }

    pub fn config(&self) -> Option<&VoiceConfig> {
        self.config.as_ref()
    }

    pub fn parent_id(&self) -> Option<&ChannelId> {
        self.parent_id.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<UserId>,
}

impl VoiceConfig {
    pub fn to_channel<'a>(
        &self,
        parser: &'a mut BotStringParser<'a>,
        mut channel: CreateChannel<'a>,
    ) -> CreateChannel<'a> {
        if let Some(name) = &self.name {
            let name = parser.render(name);
            channel = channel.name(name);
        }
        if let Some(bitrate) = self.bitrate {
            channel = channel.bitrate(bitrate);
        }
        if let Some(user_limit) = self.user_limit {
            channel = channel.user_limit(user_limit);
        }
        if let Some(user_id) = &self.locked {
            let permissions = vec![
                PermissionOverwrite {
                    allow: Permissions::CONNECT,
                    deny: Permissions::SEND_TTS_MESSAGES,
                    kind: PermissionOverwriteType::Member(*user_id),
                },
                PermissionOverwrite {
                    allow: Permissions::VIEW_CHANNEL,
                    deny: Permissions::CONNECT,
                    kind: PermissionOverwriteType::Role(parser.guild().id.everyone_role()),
                },
            ];
            channel = channel.permissions(permissions);
        }
        channel
    }
}

pub fn test_voice_hub() -> BotHash<GuildId, VoiceMasterConfig> {
    let mut voice_hub = BotHash::new();
    let mut config = VoiceMasterConfig::new(vec![MasterVoiceChannel::new(
        ChannelId::from(851183230359306251),
        None,
    )]);
    config.set_config(VoiceConfig {
        name: Some("🔊 {user.display_name}'s Channel".to_string()),
        bitrate: None,
        user_limit: Some(5),
        locked: None,
    });
    config.set_parent_id(ChannelId::from(1413051787308437584));
    voice_hub.insert(GuildId::from(851102546470371338), config);
    voice_hub
}
