use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serenity::{
    all::{GuildId, UserId},
    prelude::TypeMapKey,
};
use tokio::sync::RwLock;

use crate::{BotHash, UserConfigHash};

pub struct UserAFK;
impl TypeMapKey for UserAFK {
    type Value = Arc<RwLock<UserConfigHash<UserAFKData>>>;
}

pub struct ServerAFKConfigRepo;
impl TypeMapKey for ServerAFKConfigRepo {
    type Value = Arc<RwLock<BotHash<GuildId, ServerAFKConfig>>>;
}

pub struct ServerAFKConfig {
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAFKData {
    pub afk_status: String,
    pub last_active: chrono::DateTime<Utc>,
}

impl UserAFKData {
    pub fn new(afk_status: String) -> Self {
        Self {
            afk_status,
            last_active: Utc::now(),
        }
    }
}

impl Default for UserAFKData {
    fn default() -> Self {
        Self::new("AFK".to_string())
    }
}
