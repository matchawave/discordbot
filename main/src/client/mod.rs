use std::sync::Arc;

use serenity::{
    Client,
    all::{ClientBuilder, CreateCommand, GatewayIntents, ShardManager},
    prelude::TypeMapKey,
};
use songbird::SerenityInit;
use utils::info;

use crate::events::Handler;

mod data;

pub use data::*;

pub async fn create_client(env: Env) -> Client {
    info!("Creating client");
    match ClientBuilder::new(env.token(), get_guild_intents())
        .event_handler(Handler)
        .register_songbird()
        .await
    {
        Ok(client) => build_data(client, env).await,
        Err(err) => panic!("Failed to create client: {}", err),
    }
}

fn get_guild_intents() -> GatewayIntents {
    GatewayIntents::GUILDS
        .union(GatewayIntents::GUILD_MESSAGES)
        .union(GatewayIntents::GUILD_MESSAGE_REACTIONS)
        .union(GatewayIntents::GUILD_VOICE_STATES)
        .union(GatewayIntents::GUILD_PRESENCES)
        .union(GatewayIntents::GUILD_MEMBERS)
        .union(GatewayIntents::GUILD_MODERATION)
        .union(GatewayIntents::GUILD_EMOJIS_AND_STICKERS)
        .union(GatewayIntents::GUILD_INTEGRATIONS)
        .union(GatewayIntents::GUILD_WEBHOOKS)
        .union(GatewayIntents::GUILD_INVITES)
        .union(GatewayIntents::GUILD_SCHEDULED_EVENTS)
        .union(GatewayIntents::MESSAGE_CONTENT)
}
