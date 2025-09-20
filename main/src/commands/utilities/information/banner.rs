use std::sync::Arc;

use serenity::{
    all::{
        Colour, CommandOptionType, CommandType, Context, CreateCommandOption, CreateEmbed, Guild,
        GuildChannel,
    },
    async_trait,
};

use utils::{CommandArguments, CommandResponse, CommandTemplate, CommandTrait, ICommand, UserType};

use crate::commands::command_user_target;

const COMMAND_NAME: &str = "banner";
const COMMAND_DESCRIPTION: &str = "Get the banner of a user";

pub struct Command;

pub fn command() -> CommandTemplate {
    let user_option = CreateCommandOption::new(
        CommandOptionType::User,
        "user",
        "The user to get the banner of",
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
        _: Option<(Guild, GuildChannel)>,
        args: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String> {
        let target = command_user_target(ctx, &args).await;
        let target = target.unwrap_or_else(|| match user {
            UserType::Member(m) => m.user.clone(),
            UserType::User(u) => u.clone(),
        });

        Ok(Some(if let Some(banner_url) = target.banner_url() {
            let embed = CreateEmbed::default()
                .title(format!("Banner of {}", target.tag()))
                .image(banner_url)
                .color(Colour::BLITZ_BLUE);
            CommandResponse::new_embeds(vec![embed])
        } else {
            CommandResponse::new_content(format!("{} does not have a banner set.", target.tag()))
        }))
    }
    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}
