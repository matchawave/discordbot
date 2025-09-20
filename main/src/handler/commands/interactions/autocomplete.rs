use serenity::{
    all::{
        CacheHttp, CommandInteraction, Context, CreateAutocompleteResponse,
        CreateInteractionResponse,
    },
    json::Value,
};
use utils::{UserType, error, warning};

use crate::Commands;

pub async fn handle(ctx: &Context, autocomplete: CommandInteraction) -> Option<String> {
    let Some(guild_id) = autocomplete.guild_id else {
        warning!(
            "Command '{}' invoked outside of a guild",
            autocomplete.data.name
        );
        return None;
    };
    let location = {
        let guild = guild_id.to_guild_cached(&ctx.cache).map(|g| g.clone());
        guild.and_then(|g| {
            let channel = g.channels.get(&autocomplete.channel_id).cloned()?;
            Some((g, channel))
        })
    };

    let user = autocomplete
        .member
        .as_ref()
        .map(|m| UserType::Member(*m.clone()))
        .unwrap_or(UserType::User(autocomplete.user.clone()));

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

    if !c.supports_autocomplete() {
        error!("Command '{}' doesn't support autocomplete", name);
        return None;
    }

    let focused = autocomplete.data.autocomplete()?;
    let Some(response) = c
        .autocomplete(ctx, user, location, focused, &autocomplete)
        .await
    else {
        error!("Failed to get autocomplete response for command '{}'", name);
        return None;
    };

    let mut options = CreateAutocompleteResponse::new();

    for (index, (name, value)) in response.iter().enumerate() {
        if index >= 25 {
            break; // Discord allows a maximum of 25 choices
        }
        options = match value {
            Value::String(s) => options.clone().add_string_choice(name, s),
            Value::Number(n) => options
                .clone()
                .add_number_choice(name, n.as_f64().unwrap_or(0.0)),
            _ => {
                error!("Unsupported autocomplete value type for command '{}'", name);
                continue;
            }
        };
    }

    if let Err(e) = autocomplete
        .create_response(ctx.http(), CreateInteractionResponse::Autocomplete(options))
        .await
    {
        error!(
            "Failed to send autocomplete response for command '{}': {}",
            name, e
        );
        return None;
    }
    Some(name)
}
