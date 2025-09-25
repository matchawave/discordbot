pub mod commands;
mod environment;
mod extras;
mod pagination;
mod permissions;
mod prefixes;
mod snipes;
mod user_afk;
mod voice_master;

use std::sync::Arc;

use dashmap::DashMap;
use utils::{Data, UserConfigHash};

use serenity::{Client, all::ShardManager, prelude::TypeMap};

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

    data.insert::<Paginations>(PaginationsMap::new());
    data
}

pub async fn build_dynamic_data(data: Data, manager: Arc<ShardManager>) {
    let mut data = data.write().await;
    data.insert::<ShardManagerContainer>(manager);
}

pub use commands::*;
pub use environment::*;
pub use extras::*;
pub use pagination::*;
pub use prefixes::*;
pub use snipes::*;
pub use user_afk::*;
pub use voice_master::*;

use crate::client::data;
