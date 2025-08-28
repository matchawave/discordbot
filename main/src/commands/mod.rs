use std::sync::Arc;

use utils::{CommandTrait, PermissionLevel};

mod example;

pub mod configuration;
pub mod fun;
pub mod integration;
pub mod security;
pub mod server;
pub mod utilities;

pub type CommandModule = (Arc<dyn CommandTrait>, Vec<PermissionLevel>);
