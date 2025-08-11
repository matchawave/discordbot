use serenity::all::{CacheHttp, Context, Message};
use utils::{CommandType, LegacyOptions, error, info};

use crate::{Commands, ElapsedTime, ServerPrefix, ServerPrefixes};

pub async fn is_command(ctx: &Context, msg: &Message) -> bool {
    let timer = ElapsedTime::new();
    let Some(guild_id) = msg.guild_id else {
        return false;
    };

    let prefix = {
        let data = ctx.data.read().await;
        let p = data
            .get::<ServerPrefixes>()
            .expect("ServerPrefixes not initialized")
            .read()
            .await;

        p.get(&ServerPrefix::Guild(guild_id)).cloned().unwrap_or(
            p.get(&ServerPrefix::Default)
                .expect("Default prefix not set")
                .clone(),
        )
    };

    if !msg.content.starts_with(&prefix) {
        info!(
            "Message does not start with prefix '{}' ({}ms)",
            prefix,
            timer.elapsed_ms()
        );
        return false;
    }

    let commands = {
        let data = ctx.data.read().await;
        data.get::<Commands>()
            .expect("Commands not initialized")
            .clone()
    };

    let mut content = msg.content.trim_start_matches(&prefix);
    let c_name = content.split_whitespace().next().unwrap_or("");
    content = content.trim_start_matches(c_name).trim();

    let Some((c, perms)) = commands.get(c_name) else {
        error!("Command '{}' not found", c_name);
        return false;
    };

    let options = LegacyOptions::parse(content);

    let message_response = match c {
        CommandType::Legacy(c) => c.legacy(ctx, msg, options),
        CommandType::SlashWithLegacy(c) => c.legacy(ctx, msg, options),
        CommandType::SlashWithLegacyAutocomplete(c) => c.legacy(ctx, msg, options),
        _ => {
            error!("Command '{}' is not a legacy command", c_name);
            return false;
        }
    }
    .await;

    match message_response {
        Ok(res) => {
            if let Err(e) = msg.channel_id.send_message(ctx.http(), res).await {
                error!("Failed to send message: {}", e);
            }
            return true;
        }
        Err(e) => error!("Failed to execute command '{}': {}", c_name, e), // TODO: Handle error
    }

    false
}
