use serenity::all::{CacheHttp, Context, CreateCommand, GuildId, Http};
use utils::{error, info};

use crate::{ElapsedTime, RegisteringCommands};

pub async fn register_commands(ctx: &Context) {
    let timer = ElapsedTime::new();
    let http = ctx.http();
    let commands = {
        let data = ctx.data.read().await;
        data.get::<RegisteringCommands>()
            .cloned()
            .unwrap_or_default()
    };

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
