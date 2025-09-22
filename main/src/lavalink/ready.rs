use lavalink_rs::{client::LavalinkClient, hook, model::events::Ready};
use serenity::prelude::TypeMap;
use tokio::sync::RwLock;
use utils::{Data, info};

#[hook]
pub async fn handle(client: LavalinkClient, session: String, event: &Ready) {
    info!("{} connection established", "Lavalink".bright_red());
    let data = client.data::<RwLock<TypeMap>>().expect("Data not found");
}
