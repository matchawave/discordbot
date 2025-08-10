use std::collections::HashMap;

use serenity::{
    all::{CreateCommand, GuildId},
    prelude::TypeMapKey,
};
use tokio::sync::RwLock;
use utils::ICommand;

pub fn register_commands() -> Vec<CreateCommand> {
    let mut commands: Vec<ICommand> = Vec::new();

    let mut output_commands = Vec::new();
    let mut commands_map = HashMap::new();

    for command in commands.iter() {
        let new_command = command.get_command();

        output_commands.push(new_command);
        commands_map.insert(command.get_name().to_string(), command.get_callbacks());
    }

    output_commands
}
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ServerPrefix {
    Guild(GuildId),
    Default,
}

pub struct ServerPrefixes;
pub type ServerPrefixesMap = HashMap<ServerPrefix, RwLock<String>>;
impl TypeMapKey for ServerPrefixes {
    type Value = RwLock<ServerPrefixesMap>;
}
