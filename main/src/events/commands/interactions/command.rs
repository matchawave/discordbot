use std::collections::HashMap;

use serenity::all::{CacheHttp, CommandInteraction, Context, CreateInteractionResponse};
use utils::{CommandArguments, UserType, error, warning};

use crate::Commands;

pub async fn handle(ctx: &Context, command: &CommandInteraction) -> Option<String> {
    let Some(guild_id) = command.guild_id else {
        warning!("Command '{}' invoked outside of a guild", command.data.name);
        return None;
    };
    let location = {
        let guild = guild_id.to_guild_cached(&ctx.cache).map(|g| g.clone());
        guild.and_then(|g| {
            let channel = g.channels.get(&command.channel_id).cloned()?;
            Some((g, channel))
        })
    };

    let user = command
        .member
        .as_ref()
        .map(|m| UserType::Member(*m.clone()))
        .unwrap_or(UserType::User(command.user.clone()));

    let commands = {
        let data = ctx.data.read().await;
        data.get::<Commands>()
            .expect("Commands not initialized")
            .clone()
    };
    let c_name = command.data.name.clone();
    let Some((c, _perms)) = commands.get(&c_name) else {
        error!("Command '{}' not found", c_name);
        return None;
    };

    if !c.is_slash() {
        warning!("Command '{}' is not a slash command", c_name);
        return None;
    }

    let options = {
        let mut hash_map = HashMap::new();
        let options = &command.data.options;
        if options.is_empty() {
            None
        } else {
            for option in options.iter() {
                hash_map.insert(option.name.clone(), option.value.clone());
            }
            Some(hash_map)
        }
    };
    let args = CommandArguments::Slash(options, command);

    match c.execute(ctx, user, location, args).await {
        Ok(r) => {
            if let Some(response) = r
                && let Err(e) = command
                    .create_response(
                        ctx.http(),
                        CreateInteractionResponse::Message(response.to_interaction_msg()),
                    )
                    .await
            {
                error!("Failed to send response to command '{}': {}", c_name, e);
                return None;
            }
            Some(c_name)
        }
        Err(e) => {
            error!("Error executing command '{}': {}", c_name, e);
            None
        }
    }
}
