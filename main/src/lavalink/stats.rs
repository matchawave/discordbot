use lavalink_rs::{client::LavalinkClient, model::events::Stats};
use serenity::{all::standard::macros::hook, prelude::TypeMap};
use tokio::sync::RwLock;
use utils::Data;

#[hook]
pub async fn handle(client: LavalinkClient, session: String, event: &Stats) {
    let data = client.data::<RwLock<TypeMap>>().expect("Data not found");
}
