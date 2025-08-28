use std::{collections::HashMap, sync::Arc};

use serenity::{all::CreateCommand, prelude::TypeMapKey};
use utils::{CommandTrait, PermissionLevel, info};

use crate::commands::{configuration, fun, integration, security, server, utilities};

pub struct Commands;
pub type CommandsMap = HashMap<String, (Arc<dyn CommandTrait>, Vec<PermissionLevel>)>;
impl TypeMapKey for Commands {
    type Value = CommandsMap;
}

pub struct RegisteringCommands;
impl TypeMapKey for RegisteringCommands {
    type Value = Vec<CreateCommand>;
}

fn commands() -> Vec<utils::CommandTemplate> {
    let mut commands = Vec::new();

    commands.extend(server::get_modules());
    commands.extend(security::get_modules());
    commands.extend(integration::get_modules());
    commands.extend(fun::get_modules());
    commands.extend(configuration::get_modules());
    commands.extend(utilities::get_modules());

    commands
}

pub fn load_commands() -> (Vec<CreateCommand>, CommandsMap) {
    info!("Loading commands...");
    let mut output_commands = Vec::new();
    let mut commands_map = CommandsMap::new();

    for (command, function) in commands().iter() {
        let func = function.clone();
        if func.is_slash() {
            output_commands.push(command.get_command());
        }
        commands_map.insert(
            command.get_name().to_string(),
            (func, command.get_permissions()),
        );
        info!("Loaded command: {}", command.get_name());
    }

    info!("Loaded {} commands", output_commands.len());
    (output_commands, commands_map)
}
