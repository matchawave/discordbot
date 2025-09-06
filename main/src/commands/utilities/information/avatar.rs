use std::sync::Arc;

use serenity::{
    all::{
        AutocompleteOption, ChannelId, CommandInteraction, CommandOptionType, CommandType, Context,
        CreateCommandOption, Guild, GuildChannel, GuildId, User,
    },
    async_trait,
};

use utils::{
    AutocompleteResponse, CommandArguments, CommandResponse, CommandTemplate, CommandTrait,
    ICommand, UserType,
};

const COMMAND_NAME: &str = "avatar";
const COMMAND_DESCRIPTION: &str = "Get the avatar of a user";

pub struct Command;

pub fn command() -> CommandTemplate {
    let user_option = CreateCommandOption::new(
        CommandOptionType::User,
        "user",
        "The user to get the avatar of",
    );
    (
        ICommand::new(
            COMMAND_NAME.to_string(),
            COMMAND_DESCRIPTION.to_string(),
            CommandType::ChatInput,
            vec![user_option],
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
        let target = match args {
            CommandArguments::Legacy(args, _) => {
                args.and_then(|arg| arg.first().and_then(|option| {}))
            }
            CommandArguments::Slash(arg, _) => {}
        };
        Ok(Some(CommandResponse::default()))
    }
    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}
