use serenity::all::{Context, Ready};
use utils::info;

use crate::Environment;

mod command_registering;
mod websocket;

pub async fn handle(ctx: Context, ready: Ready) {
    info!("Bot is connected as {}", ready.user.name);
    tokio::spawn(websocket::run(ctx.data.clone(), ready.user.id.to_string()));
    command_registering::register_commands(&ctx).await;
}
