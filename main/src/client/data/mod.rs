mod commands;
mod environment;
mod extras;
mod permissions;
mod prefixes;
mod user_afk;
mod voice_master;

use std::{collections::HashMap, sync::Arc};

use serenity::Client;

pub async fn build_data(client: Client, env: Env) -> Client {
    let (commands_vec, commands_map) = commands::load_commands();
    let data = client.data.clone();
    let mut data = data.write().await;
    data.insert::<ServerPrefixes>(prefixes::setup().into());
    data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    data.insert::<Environment>(env);
    data.insert::<Commands>(commands_map);
    data.insert::<RegisteringCommands>(commands_vec);
    data.insert::<VoiceHub>(Arc::new(HashMap::new().into()));
    data.insert::<UserAFK>(Arc::new(HashMap::new().into()));
    client
}

pub use commands::{Commands, RegisteringCommands};
pub use environment::{Env, Environment, LavalinkEnv};
pub use extras::*;
pub use prefixes::{ServerPrefix, ServerPrefixes};
pub use user_afk::{AFKAccess, UserAFK, UserAFKData, UserAFKRepo};
pub use voice_master::{VoiceHub, VoiceMasterConfig};
