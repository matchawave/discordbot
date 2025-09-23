use std::sync::Arc;

use dashmap::DashMap;
use serenity::{
    all::{ChannelId, GuildId, Message, Reaction},
    prelude::TypeMapKey,
};

pub struct Snipes;
impl TypeMapKey for Snipes {
    type Value = Arc<DashMap<ChannelId, Vec<Message>>>;
}

pub struct EditSnipes;
impl TypeMapKey for EditSnipes {
    type Value = Arc<DashMap<ChannelId, Vec<Message>>>;
}

pub struct ReactionSnipes;
impl TypeMapKey for ReactionSnipes {
    type Value = Arc<DashMap<ChannelId, Vec<Reaction>>>;
}

pub struct BlacklistedSnipes;
impl TypeMapKey for BlacklistedSnipes {
    type Value = Arc<DashMap<GuildId, Vec<ChannelId>>>;
}
