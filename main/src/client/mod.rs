use std::sync::Arc;

use serenity::{
    Client,
    all::{ClientBuilder, GatewayIntents, ShardManager},
    prelude::TypeMapKey,
};
use songbird::SerenityInit;

use crate::{Env, Environment, events::Handler, info};

mod commands;
mod permissions;

pub use commands::*;
pub use permissions::*;

pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

pub async fn create_client(env: Env) -> Client {
    info!("Creating client");
    match ClientBuilder::new(env.token(), get_guild_intents())
        .event_handler(Handler)
        .register_songbird()
        .await
    {
        Ok(client) => {
            let data = client.data.clone();
            let mut data = data.write().await;
            data.insert::<ShardManagerContainer>(client.shard_manager.clone());
            data.insert::<Environment>(env);
            client
        }
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
}
