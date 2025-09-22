use std::sync::Arc;

use serenity::{
    all::{
        Colour, CommandType, Context, CreateEmbed, FormattedTimestamp, FormattedTimestampStyle,
        Guild, GuildChannel, VerificationLevel,
    },
    async_trait,
};

use utils::{CommandArguments, CommandResponse, CommandTemplate, CommandTrait, ICommand, UserType};

use crate::ShardManagerContainer;

const COMMAND_NAME: &str = "serverinfo";
const COMMAND_DESCRIPTION: &str = "View information about the server";

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
        _: UserType,
        channel: Option<(Guild, GuildChannel)>,
        _: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String> {
        let Some((guild, _)) = channel else {
            return Err("This command can only be used in a server.".to_string());
        };

        let mut fields = vec![];
        let description = get_description(ctx, &guild).await;
        fields.push(("Owner", format!("<@{}>", guild.owner_id), true));
        fields.push(("Members", get_members(&guild), true));
        fields.push(("Information", get_information(&guild), true));
        let channels = get_channels(&guild);
        let channel_title = format!("Channels ({}):", channels.0);
        if let Some(design) = get_design(&guild) {
            fields.push(("Design", design, true));
        }
        fields.push((&channel_title, channels.1, true));
        fields.push(("Counts", get_counts(&guild), true));

        let mut embed = CreateEmbed::default()
            .title(format!("{} ({})", guild.name, guild.id))
            .description(description)
            .color(Colour::BLITZ_BLUE)
            .fields(fields);

        if let Some(icon) = &guild.icon_url() {
            embed = embed.thumbnail(icon);
        }
        if let Some(banner) = &guild.banner_url() {
            embed = embed.image(banner);
        }

        Ok(Some(CommandResponse::new_embeds(vec![embed])))
    }

    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}

async fn get_description(ctx: &Context, guild: &Guild) -> String {
    let shard_manager = {
        let data = ctx.data.read().await;
        data.get::<ShardManagerContainer>()
            .cloned()
            .expect("Failed to get shard manager.")
    };
    let total_shards = { shard_manager.runners.lock().await.len() };

    let created_at = guild.id.created_at();
    let created_at = format!(
        "Server created on **{}** ({})",
        created_at.format("%B %d, %Y"), // format for Month Day, Year
        FormattedTimestamp::new(created_at, Some(FormattedTimestampStyle::RelativeTime))
    );
    let shard_info = format!(
        "__{}__ is on shard **{}/{}**",
        guild.name,
        ctx.shard_id.get() + 1,
        total_shards
    );
    format!("{}\n{}", created_at, shard_info)
}
fn get_members(guild: &Guild) -> String {
    let members = guild.members.len();
    let (humans, bots) =
        guild.members.iter().fold(
            (0, 0),
            |(h, b), (_, m)| {
                if m.user.bot { (h, b + 1) } else { (h + 1, b) }
            },
        );
    format!(
        "**Total:** {}\n**Humans:** {}\n**Bots:** {}",
        members, humans, bots
    )
}
fn get_information(guild: &Guild) -> String {
    let verification = match guild.verification_level {
        VerificationLevel::Higher => "Highest",
        VerificationLevel::High => "High",
        VerificationLevel::Medium => "Medium",
        VerificationLevel::Low => "Low",
        VerificationLevel::None => "None",
        _ => "Unknown",
    };
    let boost_level = match guild.premium_tier {
        serenity::all::PremiumTier::Tier0 => "None",
        serenity::all::PremiumTier::Tier1 => "1",
        serenity::all::PremiumTier::Tier2 => "2",
        serenity::all::PremiumTier::Tier3 => "3",
        _ => "Unknown",
    };
    let mut output = format!(
        "**Verification Level:** {}\n**Boost Level:** {}",
        verification, boost_level,
    );
    if let Some(count) = guild.premium_subscription_count
        && count > 0
    {
        output.push_str(&format!(" ({})", count));
    }
    if let Some(vanity) = &guild.vanity_url_code {
        output.push_str(&format!("\n**Vanity URL:** discord.gg/{}", vanity));
    }

    output
}
fn get_design(guild: &Guild) -> Option<String> {
    let mut output = String::new();

    if let Some(splash) = &guild.splash_url() {
        output += &format!("[Splash Image]({})", splash);
    }
    if let Some(banner) = &guild.banner_url() {
        if !output.is_empty() {
            output += "\n";
        }
        output += &format!("[Banner Image]({})", banner);
    }
    if let Some(icon) = &guild.icon_url() {
        if !output.is_empty() {
            output += "\n";
        }
        output += &format!("[Icon Image]({})", icon);
    }
    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}
fn get_channels(guild: &Guild) -> (usize, String) {
    let total = guild.channels.len();
    let (text, voice, category) =
        guild
            .channels
            .iter()
            .fold((0, 0, 0), |(t, v, c), (_, ch)| match ch.kind {
                serenity::all::ChannelType::Text => (t + 1, v, c),
                serenity::all::ChannelType::Voice => (t, v + 1, c),
                serenity::all::ChannelType::Category => (t, v, c + 1),
                _ => (t, v, c),
            });
    (
        total,
        format!(
            "**Text:** {}\n**Voice:** {}\n**Categories:** {}",
            text, voice, category
        ),
    )
}
fn get_counts(guild: &Guild) -> String {
    let roles = guild.roles.len();
    let emojis = guild.emojis.len();
    let stickers = guild.stickers.len();
    format!(
        "**Roles:** {}\n**Emojis:** {}\n**Stickers:** {}",
        roles, emojis, stickers
    )
}
