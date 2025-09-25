mod commands;
mod hash;
mod logging;
mod pagination;
mod parser;
mod permissions;

use std::sync::Arc;

use serenity::{all::UserId, prelude::TypeMap};
use tokio::sync::RwLock;

pub use commands::*;
pub use hash::*;
pub use logging::*;
pub use pagination::*;
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

pub fn parse_button_id(custom_id: &str) -> Option<(&str, u64, UserId, PaginationAction)> {
    let parts: Vec<&str> = custom_id.split('|').collect();
    if parts.len() != 4 {
        return None;
    }

    let section = parts[0];
    let id = parts[1].parse::<u64>().ok()?;
    let userid = parts[2].parse::<UserId>().ok()?;
    let action = PaginationAction::from(parts[3]);

    Some((section, id, userid, action))
}
