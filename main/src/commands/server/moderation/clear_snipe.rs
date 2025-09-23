use std::sync::Arc;

use serenity::{
    all::{CacheHttp, CommandType, Context, Guild, GuildChannel, ReactionType},
    async_trait,
};

use utils::{CommandArguments, CommandResponse, CommandTemplate, CommandTrait, ICommand, UserType};

use crate::{EditSnipes, ReactionSnipes, Snipes};

const COMMAND_NAME: &str = "clearsnipe";
const COMMAND_DESCRIPTION: &str = "Clears the stored snipe data for the server.";

pub struct Command;

pub fn command() -> CommandTemplate {
    (
        ICommand::new(
            COMMAND_NAME.to_string(),
            COMMAND_DESCRIPTION.to_string(),
            CommandType::ChatInput,
            vec![],
            vec![],
        ),
        Arc::new(Command),
    )
}

#[async_trait]
impl CommandTrait for Command {
    async fn execute<'a>(
        &self,
        ctx: &'a Context,
        user: UserType,
        channel: Option<(Guild, GuildChannel)>,
        args: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String> {
        let Some((_, channel)) = channel else {
            return Err("This command can only be used in a server.".to_string());
        };

        let (snipes, edit_snipes, reaction_snipes) = {
            let data = ctx.data.read().await;
            let snipes = data
                .get::<Snipes>()
                .cloned()
                .ok_or("Failed to get snipe data.".to_string())?;
            let edit_snipes = data
                .get::<EditSnipes>()
                .cloned()
                .ok_or("Failed to get edit snipe data.".to_string())?;
            let reaction_snipes = data
                .get::<ReactionSnipes>()
                .cloned()
                .ok_or("Failed to get reaction snipe data.".to_string())?;
            (snipes, edit_snipes, reaction_snipes)
        };

        if !snipes.contains_key(&channel.id)
            && !edit_snipes.contains_key(&channel.id)
            && !reaction_snipes.contains_key(&channel.id)
        {
            return Ok(Some(
                CommandResponse::new_content("ℹ️ There are no snipes.".to_string()).ephemeral(),
            ));
        }

        snipes.remove(&channel.id);
        edit_snipes.remove(&channel.id);
        reaction_snipes.remove(&channel.id);

        match args {
            CommandArguments::Slash(_, _) => {
                return Ok(Some(
                    CommandResponse::new_content(
                        "✅ Cleared snipe data for this channel.".to_string(),
                    )
                    .ephemeral(),
                ));
            }
            CommandArguments::Legacy(_, msg) => {
                if let Err(e) = msg
                    .react(ctx.http(), ReactionType::Unicode("✅".into()))
                    .await
                {
                    Err(format!("Failed to react to message: {}", e))?;
                }
                return Ok(None);
            }
        }
    }
    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}
