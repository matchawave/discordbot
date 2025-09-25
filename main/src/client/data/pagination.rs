use serenity::{
    all::{CreateActionRow, CreateEmbed},
    prelude::TypeMapKey,
};
use std::{
    collections::{HashMap, hash_map::Iter},
    sync::Arc,
};
use tokio::sync::RwLock;
use utils::Pagination;

pub struct Paginations;
#[derive(Clone, Debug, Default)]
pub struct PaginationsMap(Arc<RwLock<HashMap<u64, Arc<RwLock<Pagination>>>>>);
impl TypeMapKey for Paginations {
    type Value = PaginationsMap;
}

impl PaginationsMap {
    pub fn new() -> Self {
        Self(PaginationsMap::default().0)
    }

    pub async fn insert(
        &self,
        embeds: Vec<CreateEmbed>,
        user_id: u64,
    ) -> (CreateEmbed, CreateActionRow) {
        let key = self.generate_key().await;
        let pagination = Pagination::new(key, user_id, embeds);
        let mut map = self.0.write().await;
        map.insert(key, Arc::new(RwLock::new(pagination.clone())));
        pagination.current()
    }

    pub async fn get(&self, key: &u64) -> Option<Arc<RwLock<Pagination>>> {
        let map = self.0.read().await;
        map.get(key).cloned()
    }

    pub async fn remove(&self, key: &u64) -> Option<Pagination> {
        let mut map = self.0.write().await;
        let pagination = map.remove(key).map(Arc::try_unwrap).and_then(|r| r.ok());
        pagination.map(|p| p.into_inner())
    }

    async fn generate_key(&self) -> u64 {
        let mut key = fastrand::u64(1000..9999);
        while self.0.read().await.contains_key(&key) {
            key = fastrand::u64(1000..9999);
        }
        key
    }

    pub async fn map(&self) -> HashMap<u64, Arc<RwLock<Pagination>>> {
        self.0.read().await.clone()
    }
}
