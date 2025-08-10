use std::{collections::HashMap, sync::Arc};

use serenity::{
    all::{CreateCommand, GuildId},
    prelude::TypeMapKey,
};
use tokio::sync::RwLock;
use utils::{CommandType, ICommand, PermissionLevel};

pub fn register_commands() -> (Vec<CreateCommand>, CommandsMap) {
    let mut commands: Vec<ICommand> = Vec::new();

    let mut output_commands = Vec::new();
    let mut commands_map = CommandsMap::new();

    for command in commands.iter() {
        let new_command = command.get_command();

        output_commands.push(new_command);
        commands_map.insert(
            command.get_name().to_string(),
            (command.get_callbacks(), command.get_permissions()),
        );
    }

    (output_commands, commands_map)
}
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

pub struct Commands;
pub type CommandsMap = HashMap<String, (CommandType, Vec<PermissionLevel>)>;
impl TypeMapKey for Commands {
    type Value = CommandsMap;
}
