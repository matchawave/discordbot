pub mod commands;
mod environment;
mod extras;
mod permissions;
mod prefixes;
mod snipes;
mod user_afk;
mod voice_master;

use dashmap::DashMap;
use utils::UserConfigHash;

use serenity::{Client, prelude::TypeMap};

pub fn initialize_type_map(env: Env, commands_map: CommandsMap) -> TypeMap {
    let mut data = TypeMap::new();

    data.insert::<Environment>(env);
    data.insert::<Commands>(commands_map);

    data.insert::<ServerPrefixes>(prefixes::setup().into());
    data.insert::<VoiceHub>(test_voice_hub().into());
    data.insert::<UserAFK>(UserConfigHash::new().into());
    data.insert::<UserVoiceConfigRepo>(UserConfigHash::new().into());

    data.insert::<Snipes>(DashMap::new().into());
    data.insert::<EditSnipes>(DashMap::new().into());
    data.insert::<ReactionSnipes>(DashMap::new().into());
    data.insert::<BlacklistedSnipes>(DashMap::new().into());
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
