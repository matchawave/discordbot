use std::sync::Arc;

use chrono::Utc;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serenity::{all::GuildId, prelude::TypeMapKey};
use tokio::sync::RwLock;
use utils::UserConfigHash;

pub struct UserAFK;
impl TypeMapKey for UserAFK {
    type Value = Arc<UserConfigHash<UserAFKData>>;
}

pub struct ServerAFKConfigRepo;
impl TypeMapKey for ServerAFKConfigRepo {
    type Value = Arc<DashMap<GuildId, ServerAFKConfig>>;
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
