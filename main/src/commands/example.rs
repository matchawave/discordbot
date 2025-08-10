use std::sync::Arc;

use serenity::{
    all::{
        CommandInteraction, Context, CreateAutocompleteResponse, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateMessage, Message,
    },
    async_trait,
};
use utils::SlashWithLegacyAutocomplete;
use utils::{
    Autocomplete, CommandExecutionType, CommandType, ICommand, InteractionCommandResult, Legacy,
    LegacyOptions, MessageResponseResult, Slash,
};

const COMMAND_NAME: &str = "";
const COMMAND_DESCRIPTION: &str = "";

#[derive(SlashWithLegacyAutocomplete)]
pub struct Command;

pub fn command<'a>() -> ICommand<'a> {
    ICommand::new(
        COMMAND_NAME,
        COMMAND_DESCRIPTION,
        vec![],
        CommandType::SlashWithLegacyAutocomplete(Arc::new(Command), vec![]),
    )
}

#[async_trait]
impl Slash for Command {
    async fn slash(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> InteractionCommandResult {
        Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(
                match command_handler(ctx, CommandExecutionType::Command(interaction)).await {
                    Ok(response) => response,
                    Err(e) => e.to_string(),
                },
            ),
        ))
    }
}

#[async_trait]
impl Legacy for Command {
    async fn legacy(
        &self,
        ctx: &Context,
        msg: &Message,
        options: Vec<LegacyOptions>,
    ) -> MessageResponseResult {
        Ok(CreateMessage::new().content(
            match command_handler(ctx, CommandExecutionType::Legacy(msg, options)).await {
                Ok(response) => response,
                Err(e) => e.to_string(),
            },
        ))
    }
}

async fn command_handler<'a>(
    ctx: &Context,
    command: CommandExecutionType<'a>,
) -> Result<String, String> {
    Ok("Command executed successfully".to_string())
}

#[async_trait]
impl Autocomplete for Command {
    async fn autocomplete(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> InteractionCommandResult {
        // Handle autocomplete logic here
        let mut response = CreateAutocompleteResponse::new();
        Ok(CreateInteractionResponse::Autocomplete(response))
    }
}
