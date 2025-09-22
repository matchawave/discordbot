use std::sync::Arc;

use serenity::all::{CreateCommand, GuildId, Http, UserId};
use utils::{error, info};

use crate::ElapsedTime;

pub async fn run(http: Arc<Http>, commands: Vec<CreateCommand>) {
    let timer = ElapsedTime::new();
    info!("Registering commands...");

    // match http.create_global_commands(&commands).await {
    //     Ok(_) => info!(
    //         "Commands successfully registered! ({}ms)",
    //         timer.elapsed_ms()
    //     ),
    //     Err(why) => error!("Failed to register commands: {:?}", why),
    // }

    let dev_guild = GuildId::from(851102546470371338);
    match dev_guild.set_commands(http, commands).await {
        Ok(_) => info!(
            "Commands successfully registered! ({}ms)",
            timer.elapsed_ms()
        ),
        Err(why) => error!("Failed to register commands: {:?}", why),
    }
}
