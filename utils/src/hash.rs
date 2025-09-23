use dashmap::{
    DashMap,
    mapref::one::{Ref, RefMut},
};
use serenity::all::{GuildId, UserId};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserGlobalType {
    Guild(GuildId, UserId),
    User(UserId),
}

#[derive(Debug, Clone)]
pub struct UserConfigHash<V>(DashMap<UserGlobalType, V>);

impl<V> UserConfigHash<V> {
    pub fn new() -> UserConfigHash<V> {
        Self(DashMap::new())
    }

    pub fn get<'a>(
        &'a self,
        guild_id: &GuildId,
        user_id: &UserId,
    ) -> Option<Ref<'a, UserGlobalType, V>> {
        self.0
            .get(&UserGlobalType::Guild(*guild_id, *user_id))
            .or(self.0.get(&UserGlobalType::User(*user_id)))
    }

    pub fn get_mut<'a>(
        &'a self,
        guild_id: &GuildId,
        user_id: &UserId,
    ) -> Option<RefMut<'a, UserGlobalType, V>> {
        self.0
            .get_mut(&UserGlobalType::Guild(*guild_id, *user_id))
            .or(self.0.get_mut(&UserGlobalType::User(*user_id)))
    }

    pub fn insert(&self, key: UserGlobalType, value: V) {
        self.0.insert(key, value);
    }

    pub fn remove(&self, guild_id: &GuildId, user_id: &UserId) -> Option<(UserGlobalType, V)> {
        self.0
            .remove(&UserGlobalType::Guild(*guild_id, *user_id))
            .or(self.0.remove(&UserGlobalType::User(*user_id)))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&self) {
        self.0.clear();
    }
}

impl<V> Default for UserConfigHash<V> {
    fn default() -> Self {
        Self(DashMap::new())
    }
}
