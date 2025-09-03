use serenity::all::{GuildId, UserId};
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct BotHash<K, V>(HashMap<K, Arc<RwLock<V>>>);

impl<K: Eq + std::hash::Hash + Debug, V> BotHash<K, V> {
    pub fn new() -> BotHash<K, V> {
        BotHash::default()
    }

    pub fn get(&self, key: &K) -> Option<Arc<RwLock<V>>> {
        self.0.get(key).cloned()
    }

    pub fn get_raw(&self, key: &K) -> Option<V> {
        self.0
            .get(key)
            .and_then(|arc_rwlock| match Arc::try_unwrap(arc_rwlock.clone()) {
                Ok(rwlock) => Some(rwlock.into_inner()),
                Err(_) => None,
            })
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.0.insert(key, Arc::new(RwLock::new(value)));
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0
            .remove(key)
            .and_then(|arc_rwlock| match Arc::try_unwrap(arc_rwlock) {
                Ok(rwlock) => Some(rwlock.into_inner()),
                Err(_) => None,
            })
    }
}

impl<K, V> Default for BotHash<K, V> {
    fn default() -> Self {
        BotHash(HashMap::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserGlobalType {
    Guild(GuildId, UserId),
    User(UserId),
}

#[derive(Debug, Clone)]
pub struct UserConfigHash<V>(BotHash<UserGlobalType, V>);

impl<V> UserConfigHash<V> {
    pub fn new() -> UserConfigHash<V> {
        Self::default()
    }

    pub fn get(&self, guild_id: &GuildId, user_id: &UserId) -> Option<Arc<RwLock<V>>> {
        self.0
            .get(&UserGlobalType::Guild(*guild_id, *user_id))
            .or(self.0.get(&UserGlobalType::User(*user_id)))
            .clone()
    }

    pub fn get_user(&self, user_id: UserId) -> Option<Arc<RwLock<V>>> {
        self.0.get(&UserGlobalType::User(user_id)).clone()
    }

    pub fn get_raw(&self, guild_id: &GuildId, user_id: &UserId) -> Option<V> {
        self.0
            .get_raw(&UserGlobalType::Guild(*guild_id, *user_id))
            .or_else(|| self.0.get_raw(&UserGlobalType::User(*user_id)))
    }

    pub fn insert(&mut self, key: UserGlobalType, value: V) {
        self.0.insert(key, value);
    }

    pub fn remove(&mut self, guild_id: &GuildId, user_id: &UserId) -> Option<V> {
        self.0
            .remove(&UserGlobalType::Guild(*guild_id, *user_id))
            .or_else(|| self.0.remove(&UserGlobalType::User(*user_id)))
    }

    pub fn remove_user(&mut self, user_id: &UserId) -> Option<V> {
        self.0.remove(&UserGlobalType::User(*user_id))
    }
}

impl<V> Default for UserConfigHash<V> {
    fn default() -> Self {
        Self(BotHash::new())
    }
}
