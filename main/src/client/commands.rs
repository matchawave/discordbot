use std::{collections::HashMap, sync::Arc};

use serenity::{
    all::{CreateCommand, GuildId, Http},
    prelude::TypeMapKey,
};
use tokio::sync::RwLock;
use utils::{CommandType, Data, ICommand, PermissionLevel, error, info};

use crate::ElapsedTime;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ServerPrefix {
    Guild(GuildId),
    Default,
}

pub struct ServerPrefixes;
pub type ServerPrefixesMap = HashMap<ServerPrefix, String>;
impl TypeMapKey for ServerPrefixes {
    type Value = RwLock<ServerPrefixesMap>;
}

pub struct Commands;
pub type CommandsMap = HashMap<String, (CommandType, Vec<PermissionLevel>)>;
impl TypeMapKey for Commands {
    type Value = CommandsMap;
}

pub struct RegisteringCommands;
impl TypeMapKey for RegisteringCommands {
    type Value = Vec<CreateCommand>;
}

pub fn load_commands() -> (Vec<CreateCommand>, CommandsMap) {
    info!("Loading commands...");
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
        info!("Loaded command: {}", command.get_name());
    }

    info!("Loaded {} commands", output_commands.len());
    (output_commands, commands_map)
}
