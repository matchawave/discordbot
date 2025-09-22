use std::sync::Arc;

use serenity::{
    all::{
        AutocompleteOption, Channel, ChannelId, Colour, CommandInteraction, CommandOptionType,
        CommandType, Context, CreateCommandOption, CreateEmbed, FormattedTimestamp,
        FormattedTimestampStyle, Guild, GuildChannel, GuildId, Member, User,
    },
    async_trait,
    futures::channel,
};

use utils::{
    AutocompleteResponse, CommandArguments, CommandResponse, CommandTemplate, CommandTrait,
    ICommand, UserType,
};

use crate::commands::command_channel_target;

const COMMAND_NAME: &str = "channel";
const COMMAND_DESCRIPTION: &str = "View information about a channel";

pub struct Command;

pub fn command() -> CommandTemplate {
    let channel_option = CreateCommandOption::new(
        CommandOptionType::Channel,
        "channel",
        "The channel to get information about",
    );
    (
        ICommand::new(
            COMMAND_NAME.to_string(),
            COMMAND_DESCRIPTION.to_string(),
            CommandType::ChatInput,
            vec![channel_option],
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
        _: UserType,
        channel: Option<(Guild, GuildChannel)>,
        args: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String> {
        let Some((guild, channel)) = channel else {
            return Err("This command can only be used in a server.".to_string());
        };
        let channel = command_channel_target(ctx, &args, &guild)
            .await
            .unwrap_or(channel);

        let mut fields = vec![];
        fields.push(("Channel ID", format!("`{}`", channel.id), true));
        fields.push(("Type", format!("`{}`", channel.kind.name()), true));
        fields.push(("Guild", format!("{} (`{}`)", guild.name, guild.id), true));
        if let Some(parent_text) = fetch_category_name(&guild, &channel) {
            fields.push(("Category", parent_text, true));
        }
        if let Some(topic) = channel.topic {
            fields.push(("Topic", topic.clone(), false));
        }
        fields.push((
            "Creation Date",
            FormattedTimestamp::new(
                channel.id.created_at(),
                Some(FormattedTimestampStyle::ShortDateTime),
            )
            .to_string(),
            false,
        ));

        let embed = CreateEmbed::default()
            .title(format!("Channel Information: {}", channel.name))
            .color(Colour::BLITZ_BLUE)
            .fields(fields);

        Ok(Some(CommandResponse::new_embeds(vec![embed])))
    }
    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}

fn fetch_category_name(guild: &Guild, channel: &GuildChannel) -> Option<String> {
    if let Some(parent_id) = channel.parent_id
        && let Some(category) = guild.channels.get(&parent_id)
    {
        return Some(format!("{} (`{}`)", category.name, parent_id));
    }
    None
}
