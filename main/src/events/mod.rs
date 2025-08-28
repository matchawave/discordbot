use serenity::{
    all::{Context, EventHandler, Guild, Interaction, Message, Ready, UnavailableGuild},
    async_trait,
};
use utils::info;

use crate::{ElapsedTime, Environment};

mod commands;
mod help;
mod ready;
mod user_afk;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        let _ = is_new;
        info!(
            "Joined guild: {} ({} members)",
            guild.name, guild.member_count
        );
    }

    async fn guild_delete(&self, ctx: Context, guild: UnavailableGuild, info: Option<Guild>) {
        match info {
            Some(g) => info!("Left guild: {} ({} members)", g.name, g.member_count),
            None => info!("Guild {} got deleted", guild.id),
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::handle(ctx, ready).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        println!("Rcv msg: {}", msg.content);
        tokio::spawn(user_afk::notify_afk_mentions(ctx.clone(), msg.clone()));
        user_afk::check_afk_status(&ctx, &msg).await;
        if help::is_asking_for_bot_prefix(&ctx, &msg).await {
            return;
        }
        if commands::message::is_command(&ctx, &msg).await {
            return;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        commands::interactions::handle(&ctx, &interaction).await;
    }
}
