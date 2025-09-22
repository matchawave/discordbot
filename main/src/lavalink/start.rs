use lavalink_rs::{client::LavalinkClient, hook, model::events::TrackStart};
use serenity::prelude::TypeMap;
use tokio::sync::RwLock;

#[hook]
pub async fn handle(client: LavalinkClient, session: String, event: &TrackStart) {
    let data = client.data::<RwLock<TypeMap>>().expect("Data not found");
}
