use lavalink_rs::{client::LavalinkClient, hook, model::events::TrackStuck};
use serenity::prelude::TypeMap;
use tokio::sync::RwLock;

#[hook]
pub async fn handle(client: LavalinkClient, session: String, event: &TrackStuck) {
    let data = client.data::<RwLock<TypeMap>>().expect("Data not found");
}
