use lavalink_rs::{
    client::LavalinkClient,
    hook,
    model::events::{Events, WebSocketClosed},
    node::NodeBuilder,
    prelude::NodeDistributionStrategy,
};
use serenity::{all::CurrentUser, prelude::TypeMapKey};
use utils::{Data, info};

use crate::LavalinkEnv;

mod end;
mod exception;
mod ready;
mod start;
mod stats;
mod stuck;
mod update;

pub struct LavaClient;
impl TypeMapKey for LavaClient {
    type Value = LavalinkClient;
}

#[derive(Clone)]
pub struct LavaLinkInstance {
    user_id: u64,
    hostname: String,
    password: String,
    data: Data,
}

impl LavaLinkInstance {
    pub fn new(user: &CurrentUser, lava_env: &LavalinkEnv, data: &Data) -> Self {
        Self {
            user_id: user.id.get(),
            hostname: lava_env.hostname().to_string(),
            password: lava_env.password().to_string(),
            data: data.clone(),
        }
    }
    pub async fn connect(&self) {
        let events = Self::get_events();
        let hostname = self.hostname.clone();
        let password = self.password.clone();
        let user_id = self.user_id.into();

        info!("Connecting to {}", "Lavalink".bright_red());

        let node = NodeBuilder {
            hostname,
            password,
            user_id,
            ..Default::default()
        };

        let client = LavalinkClient::new_with_data(
            events,
            vec![node],
            NodeDistributionStrategy::round_robin(),
            self.data.clone(),
        )
        .await;

        self.data.write().await.insert::<LavaClient>(client);
    }

    fn get_events() -> Events {
        Events {
            ready: Some(ready::handle),
            track_start: Some(start::handle),
            track_end: Some(end::handle),
            track_exception: Some(exception::handle),
            track_stuck: Some(stuck::handle),
            player_update: Some(update::handle),
            stats: Some(stats::handle),
            websocket_closed: Some(Self::ws_closed),
            raw: None,
        }
    }

    #[hook]
    async fn ws_closed(client: LavalinkClient, session: String, event: &WebSocketClosed) {}
}
