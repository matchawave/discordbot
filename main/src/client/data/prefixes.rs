use std::{collections::HashMap, sync::Arc};

use serenity::{all::GuildId, prelude::TypeMapKey};
use tokio::sync::RwLock;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ServerPrefix {
    Guild(GuildId),
    Default,
}

pub struct ServerPrefixes;
pub type ServerPrefixesMap = HashMap<ServerPrefix, String>;
impl TypeMapKey for ServerPrefixes {
    type Value = Arc<RwLock<ServerPrefixesMap>>;
}

pub fn setup() -> ServerPrefixesMap {
    let mut prefixes = ServerPrefixesMap::new();
    prefixes.insert(ServerPrefix::Default, "!".to_string());
    prefixes
}
