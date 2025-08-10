use std::collections::HashMap;

use serenity::{
    all::{GuildId, RoleId},
    prelude::TypeMapKey,
};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub enum PermissionLevel {
    BotMaster,
    Administrator,
    Moderator,
    User,
}

pub struct BotMasterRoles;
pub type BotMasterRolesMap = HashMap<GuildId, RwLock<Vec<RoleId>>>;
impl TypeMapKey for BotMasterRoles {
    type Value = RwLock<BotMasterRolesMap>;
}

pub struct AdministratorRoles;
pub type AdministratorRolesMap = HashMap<GuildId, RwLock<Vec<RoleId>>>;
impl TypeMapKey for AdministratorRoles {
    type Value = RwLock<AdministratorRolesMap>;
}

pub struct ModeratorRoles;
pub type ModeratorRolesMap = HashMap<GuildId, RwLock<Vec<RoleId>>>;
impl TypeMapKey for ModeratorRoles {
    type Value = RwLock<ModeratorRolesMap>;
}
