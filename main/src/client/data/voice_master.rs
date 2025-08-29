use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use serenity::{
    all::{ChannelId, GuildId, UserId},
    prelude::TypeMapKey,
};
use tokio::sync::RwLock;

use crate::{BotHash, UserConfigHash};

pub struct VoiceHub;
impl TypeMapKey for VoiceHub {
    type Value = Arc<RwLock<BotHash<GuildId, VoiceMasterConfig>>>;
}

pub struct UserVoiceConfigRepo;
impl TypeMapKey for UserVoiceConfigRepo {
    type Value = Arc<RwLock<UserConfigHash<UserVoiceConfig>>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceMasterConfig {
    master: Vec<MasterVoiceChannel>,
    active: Vec<ActiveVoiceChannel>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVoiceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
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
}
