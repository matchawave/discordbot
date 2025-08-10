use serenity::{
    all::{Context, EventHandler, Guild, Interaction, Message, Ready, UnavailableGuild},
    async_trait,
};
use utils::info;

use crate::{ElapsedTime, Environment};

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

    async fn message(&self, ctx: Context, msg: Message) {
        if commands::message::is_command(&ctx, &msg).await {
            return;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        commands::interactions::handle(&ctx, &interaction).await;
    }
}
