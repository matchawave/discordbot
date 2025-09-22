use std::collections::HashMap;

use serenity::{
    all::{GuildId, RoleId},
    prelude::TypeMapKey,
};
use tokio::sync::RwLock;

pub struct PermissionMap;
pub type BotMasterRolesMap = HashMap<GuildId, RwLock<Vec<RoleId>>>;
impl TypeMapKey for PermissionMap {
    type Value = RwLock<BotMasterRolesMap>;
}
