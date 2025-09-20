use std::sync::{Arc, RwLock};

use serenity::{
    all::{ChannelId, Message},
    prelude::TypeMapKey,
};
use utils::BotHash;

pub struct Snipes;
impl TypeMapKey for Snipes {
    type Value = Arc<RwLock<BotHash<ChannelId, Vec<Message>>>>;
}

pub struct EditSnipes;
impl TypeMapKey for EditSnipes {
    type Value = Arc<RwLock<BotHash<ChannelId, Vec<Message>>>>;
}

pub struct ReactionSnipes;
impl TypeMapKey for ReactionSnipes {
    type Value = Arc<RwLock<BotHash<ChannelId, Vec<Message>>>>;
}
