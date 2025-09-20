mod commands;
mod hash;
mod logging;
mod parser;
mod permissions;

use std::sync::Arc;

use serenity::{all::Permissions, prelude::TypeMap};
use tokio::sync::RwLock;

pub use commands::*;
pub use hash::*;
pub use parser::*;
pub use permissions::*;

pub type InteractionCommandResult = Result<serenity::builder::CreateInteractionResponse, String>;
pub type MessageResponseResult = Result<serenity::builder::CreateMessage, String>;

pub type Data = Arc<RwLock<TypeMap>>;

pub fn position_suffix(position: usize) -> String {
    let suffix = match position % 10 {
        1 if position % 100 != 11 => "st",
        2 if position % 100 != 12 => "nd",
        3 if position % 100 != 13 => "rd",
        _ => "th",
    };
    format!("{}{}", position, suffix)
}
