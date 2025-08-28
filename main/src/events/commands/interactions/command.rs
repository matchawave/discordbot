use std::collections::HashMap;

use serenity::all::{CacheHttp, CommandInteraction, Context, CreateInteractionResponse};
use utils::{CommandArguments, error, warning};

use crate::Commands;

pub async fn handle(ctx: &Context, command: &CommandInteraction) -> Option<String> {
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
    let channel = command.channel_id;
    let guild_and_channel = command.guild_id.map(|g| (g, channel));
    // TODO: Check User Permissions

    if let Some(response) = c.execute(ctx, &command.user, guild_and_channel, args).await {
        if let Err(e) = command
            .create_response(
                ctx.http(),
                CreateInteractionResponse::Message(response.to_interaction_msg()),
            )
            .await
        {
            error!("Failed to send response to command '{}': {}", c_name, e);
            return None;
        }
    }

    Some(c_name)
}
