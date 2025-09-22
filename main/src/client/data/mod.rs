pub mod commands;
mod environment;
mod extras;
mod permissions;
mod prefixes;
mod snipes;
mod user_afk;
mod voice_master;

use std::sync::Arc;
use utils::{BotHash, UserConfigHash};

use serenity::{Client, prelude::TypeMap};

pub fn initialize_type_map(env: Env, commands_map: CommandsMap) -> TypeMap {
    let mut data = TypeMap::new();

    data.insert::<Environment>(env);
    data.insert::<Commands>(commands_map);

    data.insert::<ServerPrefixes>(Arc::new(prefixes::setup().into()));
    data.insert::<VoiceHub>(Arc::new(test_voice_hub().into()));
    data.insert::<UserAFK>(Arc::new(UserConfigHash::new().into()));
    data.insert::<UserVoiceConfigRepo>(Arc::new(UserConfigHash::new().into()));

    data.insert::<Snipes>(Arc::new(BotHash::new().into()));
    data.insert::<EditSnipes>(Arc::new(BotHash::new().into()));
    data.insert::<ReactionSnipes>(Arc::new(BotHash::new().into()));
    data
}

pub async fn build_dynamic_data(client: Client) -> Client {
    let data = client.data.clone();
    let mut data = data.write().await;

    data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    client
}

pub use commands::*;
pub use environment::*;
pub use extras::*;
pub use prefixes::*;
pub use snipes::*;
pub use user_afk::*;
pub use voice_master::*;
