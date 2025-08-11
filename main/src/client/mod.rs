use std::{collections::HashMap, sync::Arc};

use serenity::{
    Client,
    all::{ClientBuilder, CreateCommand, GatewayIntents, ShardManager},
    prelude::TypeMapKey,
};
use songbird::SerenityInit;
use utils::{CommandType, info};

use crate::{Env, Environment, events::Handler};

mod commands;
mod permissions;

pub use commands::*;
pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

pub async fn create_client(env: Env) -> Client {
    let (commands_vec, commands_map) = commands::load_commands();
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
            data.insert::<Commands>(commands_map);
            data.insert::<RegisteringCommands>(commands_vec);
            data.insert::<ServerPrefixes>(setup_prefixes().into());
            info!("Client created successfully");
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

fn setup_prefixes() -> ServerPrefixesMap {
    let mut prefixes = ServerPrefixesMap::new();
    prefixes.insert(ServerPrefix::Default, "!".to_string());
    prefixes
}
