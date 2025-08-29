pub mod commands;
pub mod events;
pub mod websocket;

mod client;
mod lavalink;
mod misc;

use std::{collections::HashMap, fmt::Debug, sync::Arc};

pub use client::*;
pub use lavalink::*;
pub use misc::*;
use serenity::all::{GuildId, UserId};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct BotHash<K, V> {
    base: HashMap<K, Arc<RwLock<V>>>,
}

impl<K: Eq + std::hash::Hash + Debug, V> BotHash<K, V> {
    pub fn new() -> BotHash<K, V> {
        Self {
            base: HashMap::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<Arc<RwLock<V>>> {
        self.base.get(key).cloned()
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.base.insert(key, Arc::new(RwLock::new(value)));
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.base
            .remove(key)
            .and_then(|arc_rwlock| match Arc::try_unwrap(arc_rwlock) {
                Ok(rwlock) => Some(rwlock.into_inner()),
                Err(_) => None,
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserGlobalType {
    Guild(GuildId, UserId),
    User(UserId),
}

#[derive(Debug, Clone)]
pub struct UserConfigHash<V> {
    base: BotHash<UserGlobalType, V>,
}

impl<V> UserConfigHash<V> {
    pub fn new() -> UserConfigHash<V> {
        Self {
            base: BotHash::new(),
        }
    }

    pub fn get(&self, guild_id: &GuildId, user_id: &UserId) -> Option<Arc<RwLock<V>>> {
        self.base
            .get(&UserGlobalType::Guild(*guild_id, *user_id))
            .or(self.base.get(&UserGlobalType::User(*user_id)))
            .clone()
    }

    pub fn get_user(&self, user_id: UserId) -> Option<Arc<RwLock<V>>> {
        self.base.get(&UserGlobalType::User(user_id)).clone()
    }

    pub fn insert(&mut self, key: UserGlobalType, value: V) {
        self.base.insert(key, value);
    }

    pub fn remove(&mut self, guild_id: &GuildId, user_id: &UserId) -> Option<V> {
        self.base
            .remove(&UserGlobalType::Guild(*guild_id, *user_id))
            .or_else(|| self.base.remove(&UserGlobalType::User(*user_id)))
    }

    pub fn remove_user(&mut self, user_id: &UserId) -> Option<V> {
        self.base.remove(&UserGlobalType::User(*user_id))
    }
}
