use serenity::all::{Context, Reaction};

use crate::snipes;

pub async fn add(ctx: Context, reaction: Reaction) {}

pub async fn remove(ctx: Context, reaction: Reaction) {
    let Some(guild_id) = reaction.guild_id else {
        return;
    };

    snipes::reaction(&ctx.data, &reaction, &guild_id).await;
}
