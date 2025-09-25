use std::sync::Arc;

use serenity::{
    all::{
        AutocompleteOption, Channel, ChannelId, CommandInteraction, CommandType, Context,
        CreateActionRow, CreateButton, CreateEmbed, Guild, GuildChannel, GuildId, Member,
        ReactionType, User,
    },
    async_trait,
};

use utils::{
    AutocompleteResponse, CommandArguments, CommandResponse, CommandTemplate, CommandTrait,
    ICommand, UserType,
};

use crate::Paginations;

const COMMAND_NAME: &str = "testpages";
const COMMAND_DESCRIPTION: &str = "Test pagination with multiple embeds.";

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
        let mut embed_list = vec![];
        for i in 1..=5 {
            let embed = CreateEmbed::default()
                .title(format!("Embed {}", i))
                .description("This is a test embed.");
            embed_list.push(embed);
        }

        let user = match user {
            UserType::User(u) => u,
            UserType::Member(m) => m.user.clone(),
        };

        let data = ctx.data.read().await;
        let pages = data
            .get::<Paginations>()
            .ok_or("Failed to get paginations data.".to_string())?
            .insert(embed_list, user.id.get())
            .await;

        Ok(Some(
            CommandResponse::new_embeds(vec![pages.0]).components(vec![pages.1]),
        ))
    }

    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}
