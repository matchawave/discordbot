use std::sync::Arc;

use serenity::{
    all::{
        AutocompleteOption, Channel, ChannelId, CommandInteraction, CommandType, Context, Guild,
        GuildChannel, GuildId, Member, User,
    },
    async_trait,
};

use utils::{
    AutocompleteResponse, CommandArguments, CommandResponse, CommandTemplate, CommandTrait,
    ICommand, UserType,
};

const COMMAND_NAME: &str = "reactionhistory";
const COMMAND_DESCRIPTION: &str = "View the reaction history for a message.";

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
        Ok(Some(CommandResponse::default()))
    }
    async fn autocomplete<'a>(
        &self,
        ctx: &'a Context,
        user: UserType,
        channel: Option<(Guild, GuildChannel)>,
        focused: AutocompleteOption<'a>,
        interaction: &'a CommandInteraction,
    ) -> Option<AutocompleteResponse> {
        // Handle autocomplete logic here
        let mut response = AutocompleteResponse::new();

        Some(response)
    }
    fn is_legacy(&self) -> bool {
        false
    }
    fn is_slash(&self) -> bool {
        true
    }
    fn supports_autocomplete(&self) -> bool {
        true
    }
}
