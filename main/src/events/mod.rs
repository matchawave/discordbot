use serenity::{
    all::{Context, EventHandler, Guild, Ready, UnavailableGuild},
    async_trait,
};

use crate::{ElapsedTime, Environment, info};

mod commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {}

    async fn guild_delete(&self, ctx: Context, guild: UnavailableGuild, info: Option<Guild>) {}

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Bot is connected as {}", ready.user.name);
        let data = ctx.data.clone();
        let env = data.read().await.get::<Environment>().cloned();
    }
}
