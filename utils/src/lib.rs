mod commands;
mod permissions;

use std::sync::Arc;

use serenity::prelude::TypeMap;
use tokio::sync::RwLock;

pub use commands::*;
pub use permissions::*;

pub type InteractionCommandResult =
    Result<serenity::builder::CreateInteractionResponse, Box<dyn std::error::Error + Send + Sync>>;
pub type MessageResponseResult =
    Result<serenity::builder::CreateMessage, Box<dyn std::error::Error + Send + Sync>>;

pub type Data = Arc<RwLock<TypeMap>>;
