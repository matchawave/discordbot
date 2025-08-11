use serenity::all::{
    CacheHttp, CommandInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use utils::{CommandType, error};

use crate::Commands;

pub async fn handle(ctx: &Context, command: &CommandInteraction) -> Option<String> {
    let commands = {
        let data = ctx.data.read().await;
        data.get::<Commands>()
            .expect("Commands not initialized")
            .clone()
    };
    let c_name = command.data.name.clone();
    let Some((c, perms)) = commands.get(&c_name) else {
        error!("Command '{}' not found", c_name);
        return None;
    };

    // TODO: Check User Permissions

    let response = match c {
        CommandType::Slash(c) => c.slash(ctx, command),
        CommandType::SlashWithAutocomplete(c) => c.slash(ctx, command),
        CommandType::SlashWithLegacy(c) => c.slash(ctx, command),
        CommandType::SlashWithLegacyAutocomplete(c) => c.autocomplete(ctx, command),
        _ => {
            error!("Command '{}' is not a slash command", c_name);
            return None;
        }
    }
    .await;

    let res = match response {
        Ok(res) => res,
        Err(e) => {
            error!("Error executing command '{}': {}", c_name, e);
            let message = CreateInteractionResponseMessage::new()
                .content(format!("Error executing command: {}", e))
                .ephemeral(true);
            CreateInteractionResponse::Message(message)
        }
    };
    if let Err(e) = command.create_response(ctx.http(), res).await {
        error!("Failed to send error to command '{}': {}", c_name, e);
        return None;
    }
    Some(c_name)
}
