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

use crate::{Paginations, Snipes, commands::server::moderation::snipe};

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
        user: UserType,
        channel: Option<(Guild, GuildChannel)>,
        args: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String> {
        let Some((_, channel)) = channel else {
            return Err("This command can only be used in a server.".to_string());
        };

        let user = match user {
            UserType::User(u) => u,
            UserType::Member(m) => m.user.clone(),
        };

        let snipes = {
            let data = ctx.data.read().await;
            data.get::<Snipes>()
                .cloned()
                .ok_or("Failed to get snipe data.".to_string())?
        };

        let mut embeds = vec![];

        if let Some(snipes) = snipes.get(&channel.id).map(|s| s.clone()) {
            for (index, msg) in snipes.iter().rev().enumerate() {
                let author = CreateEmbedAuthor::new(&msg.author.name).icon_url(
                    msg.author
                        .avatar_url()
                        .unwrap_or(msg.author.default_avatar_url()),
                );

                let length = if snipes.len() > 1 {
                    format!("{} messages", snipes.len())
                } else {
                    "1 message".to_string()
                };
                let footer = CreateEmbedFooter::new(format!("{}/{}", index + 1, length));

                let content = &msg.content;
                let timestamp = msg.timestamp;
                let mut embed = CreateEmbed::default()
                    .author(author)
                    .description(content)
                    .timestamp(timestamp)
                    .footer(footer)
                    .color(Colour::RED);

                if let Some(attachment) = msg.attachments.first() {
                    embed = embed.image(attachment.url.clone());
                }

                embeds.push(embed);
            }
        }

        let mut response = CommandResponse::default();

        if embeds.is_empty() {
            let embed = CreateEmbed::default()
                .title("No Snipes Found")
                .description("There are no snipes to view in this channel.")
                .color(Colour::RED);
            response = response.embeds(vec![embed]);
        } else if embeds.len() > 1 {
            let data = ctx.data.read().await;
            let pages = data
                .get::<Paginations>()
                .ok_or("Failed to get paginations data.".to_string())?
                .insert(embeds, user.id.get())
                .await;
            response = response.embeds(vec![pages.0]).components(vec![pages.1]);
        } else {
            response = response.embeds(vec![embeds[0].clone()]);
        };

        Ok(Some(response.reply()))
    }

    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}
