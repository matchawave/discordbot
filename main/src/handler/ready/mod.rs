use serenity::all::{Context, Ready};
use utils::info;

use crate::{Environment, LavaLinkInstance, websocket::WebSocketInstance};

mod command_registering;

pub async fn handle(ctx: Context, ready: Ready) {
    info!("Bot is connected as {}", ready.user.name);
    let data = ctx.data.clone();
    let user = ready.user;

    let environment = {
        let data = data.read().await;
        data.get::<Environment>().cloned().unwrap_or_default()
    };

    let websocket = WebSocketInstance::new(&user, &environment, &data);

    let lava_env = environment.lavalink().await;
    let lavalink = LavaLinkInstance::new(&user, lava_env, &data);

    // websocket.connect().await;
    // lavalink.connect().await;
    command_registering::run(&ctx).await;
}
