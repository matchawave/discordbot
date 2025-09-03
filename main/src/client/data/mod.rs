mod commands;
mod environment;
mod extras;
mod permissions;
mod prefixes;
mod user_afk;
mod voice_master;

use std::sync::Arc;
use utils::{BotHash, UserConfigHash};

use serenity::{Client, all::GuildId};

pub async fn build_data(client: Client, env: Env) -> Client {
    let (commands_vec, commands_map) = commands::load_commands();
    let data = client.data.clone();
    let mut data = data.write().await;
    data.insert::<ServerPrefixes>(prefixes::setup().into());
    data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    data.insert::<Environment>(env);
    data.insert::<Commands>(commands_map);
    data.insert::<RegisteringCommands>(commands_vec);

    data.insert::<VoiceHub>(Arc::new(test_voice_hub().into()));
    data.insert::<UserAFK>(Arc::new(UserConfigHash::new().into()));
    data.insert::<UserVoiceConfigRepo>(Arc::new(UserConfigHash::new().into()));
    client
}

pub use commands::{Commands, RegisteringCommands};
pub use environment::{Env, Environment, LavalinkEnv};
pub use extras::*;
pub use prefixes::{ServerPrefix, ServerPrefixes};
pub use user_afk::*;
pub use voice_master::*;
