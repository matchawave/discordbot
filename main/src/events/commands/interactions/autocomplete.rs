use serenity::all::{CacheHttp, CommandInteraction, Context};
use utils::{CommandType, error};

use crate::Commands;

pub async fn handle(ctx: &Context, autocomplete: &CommandInteraction) -> Option<String> {
    let name = autocomplete.data.name.clone();
    let commands = {
        let data = ctx.data.read().await;
        data.get::<Commands>()
            .expect("Commands not initialized")
            .clone()
    };

    let Some((c, _perms)) = commands.get(&name) else {
        error!("Command '{}' doesn't support autocomplete", name);
        return None;
    };

    let response = match c {
        CommandType::Autocomplete(c) => c.autocomplete(ctx, autocomplete),
        CommandType::SlashWithAutocomplete(c) => c.autocomplete(ctx, autocomplete),
        CommandType::SlashWithLegacyAutocomplete(c) => c.autocomplete(ctx, autocomplete),
        _ => {
            error!("Unsupported command type for autocomplete: {}", name);
            return None;
        }
    }
    .await;

    let res = match response {
        Ok(res) => res,
        Err(e) => {
            error!("Error executing autocomplete for command '{}': {}", name, e);
            return None;
        }
    };

    if let Err(e) = autocomplete.create_response(ctx.http(), res).await {
        error!(
            "Failed to send autocomplete response for command '{}': {}",
            name, e
        );
        return None;
    }

    Some(name)
}
