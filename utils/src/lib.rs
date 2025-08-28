mod commands;
mod logging;
mod permissions;

use std::sync::Arc;

use serenity::prelude::TypeMap;
use tokio::sync::RwLock;

pub use commands::*;
pub use permissions::*;

pub type InteractionCommandResult = Result<serenity::builder::CreateInteractionResponse, String>;
pub type MessageResponseResult = Result<serenity::builder::CreateMessage, String>;

pub type Data = Arc<RwLock<TypeMap>>;
