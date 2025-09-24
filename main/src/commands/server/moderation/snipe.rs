use std::{sync::Arc, vec};

use serenity::{
    all::{
        Colour, CommandType, Context, CreateActionRow, CreateButton, CreateCommandOption,
        CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Guild, GuildChannel, ReactionType,
    },
    async_trait,
};

use utils::{
    BotPermission, CommandArguments, CommandResponse, CommandTemplate, CommandTrait, ICommand,
    UserType,
};

use crate::Snipes;

const COMMAND_NAME: &str = "snipe";
const COMMAND_DESCRIPTION: &str = "View the last deleted message in this channel.";

pub struct Command;

pub fn command() -> CommandTemplate {
    (
        ICommand::new(
            COMMAND_NAME.to_string(),
            COMMAND_DESCRIPTION.to_string(),
            CommandType::ChatInput,
            vec![],
            vec![BotPermission::ManageMessages],
        ),
        Arc::new(Command),
    )
}

#[async_trait]
impl CommandTrait for Command {
    async fn execute<'a>(
        &self,
        ctx: &'a Context,
        _: UserType,
        channel: Option<(Guild, GuildChannel)>,
        args: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String> {
        let Some((_, channel)) = channel else {
            return Err("This command can only be used in a server.".to_string());
        };

        let snipes = {
            let data = ctx.data.read().await;
            data.get::<Snipes>()
                .cloned()
                .ok_or("Failed to get snipe data.".to_string())?
        };
        let mut embed: CreateEmbed = CreateEmbed::default();

        if let Some(snipes) = snipes.get(&channel.id)
            && !snipes.is_empty()
            && let Some(first) = snipes.first()
        {
            let author = CreateEmbedAuthor::new(&first.author.name).icon_url(
                first
                    .author
                    .avatar_url()
                    .unwrap_or(first.author.default_avatar_url()),
            );
            let length = if snipes.len() > 1 {
                format!("{} messages", snipes.len())
            } else {
                "1 message".to_string()
            };
            let footer = CreateEmbedFooter::new(format!("{}/{}", 1, length));

            let content = &first.content;
            let timestamp = first.timestamp;
            embed = embed
                .clone()
                .author(author)
                .description(content)
                .timestamp(timestamp)
                .footer(footer)
                .color(Colour::ROSEWATER);
        } else {
            embed = embed
                .clone()
                .title("No Snipes Found")
                .description("There are no snipes to view in this channel.")
                .color(Colour::ROSEWATER);
        }
        let mut response = CommandResponse::new_embeds(vec![embed]);

        Ok(Some(response))
    }

    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}
