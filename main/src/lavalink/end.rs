use lavalink_rs::{client::LavalinkClient, model::events::TrackEnd};
use serenity::{all::standard::macros::hook, prelude::TypeMap};
use tokio::sync::RwLock;

#[hook]
pub async fn handle(client: LavalinkClient, session: String, event: &TrackEnd) {
    let data = client.data::<RwLock<TypeMap>>().expect("Data not found");
}
