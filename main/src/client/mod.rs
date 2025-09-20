use std::time::Duration;

use serenity::{
    Client,
    all::{ClientBuilder, GatewayIntents, Settings},
};
use songbird::SerenityInit;
use utils::info;

mod data;

pub use data::*;

use crate::events::Handler;

pub async fn create_client(env: Env) -> Client {
    info!("Creating client");
    match ClientBuilder::new(env.token(), get_guild_intents())
        .raw_event_handler(Handler::new())
        .cache_settings(get_settings())
        .type_map(initialize_type_map(env))
        .register_songbird()
        .await
    {
        Ok(client) => build_dynamic_data(client).await,
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

fn get_settings() -> Settings {
    let mut settings = Settings::default();
    settings.max_messages = 10_000;
    settings.time_to_live = Duration::from_secs(12 * 60 * 60); // 12 hours
    settings
}
