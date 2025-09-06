use std::{any::TypeId, collections::HashMap, fmt::Display, process, sync::Arc};

use chrono::{Duration, TimeDelta};
use serenity::{
    all::{
        create_poll::Ready, AutocompleteOption, Channel, ChannelId, CommandDataOptionValue, CommandInteraction, CommandType, Context, CreateActionRow, CreateAttachment, CreateCommandOption, CreateEmbed, CreateInteractionResponseMessage, CreateMessage, CreatePoll, Guild, GuildChannel, GuildId, Member, Message, Role, RoleId, User, UserId
    },
    async_trait,
    json::Value,
};

use crate::{PermissionLevel, error, warning};

#[async_trait]
pub trait CommandTrait: Send + Sync {
    async fn execute<'a>(
        &self,
        ctx: &'a Context,
        user: UserType,
        location: Option<(Guild, GuildChannel)>,
        args: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String>;
    async fn autocomplete<'a>(
        &self,
        ctx: &'a Context,
        user: UserType,
        location: Option<(Guild, GuildChannel)>,
        focused: AutocompleteOption<'a>,
        interaction: &'a CommandInteraction,
    ) -> Option<AutocompleteResponse> {
        None
    }
    fn is_legacy(&self) -> bool {
        false
    }
    fn is_slash(&self) -> bool {
        false
    }
    fn supports_autocomplete(&self) -> bool {
        false
    }
}

pub struct ICommand {
    name: String,
    description: String,
    options: Vec<CreateCommandOption>,
    permissions: Vec<PermissionLevel>,
    kind: CommandType,
}

impl ICommand {
    pub fn new(
        name: String,
        description: String,
        kind: CommandType,
        options: Vec<CreateCommandOption>,
        permissions: Vec<PermissionLevel>,
    ) -> Self {
        if options.len() >= 25 {
            error!("Command {} has too many options", name);
            process::exit(1);
        }
        if description.len() > 100 {
            error!("Command {} has too long description", name);
            process::exit(1);
        }
        if name.len() > 32 {
            error!("Command {} has too long name", name);
            process::exit(1);
        }

        if name.is_empty() {
            error!("Command name cannot be empty");
            process::exit(1);
        }
        if description.is_empty() {
            error!("Command description cannot be empty");
            process::exit(1);
        }

        Self {
            name,
            description,
            options,
            permissions,
            kind,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_options(&self) -> &[serenity::all::CreateCommandOption] {
        &self.options
    }

    pub fn get_command(&self) -> serenity::all::CreateCommand {
        if self.options.is_empty() {
            serenity::all::CreateCommand::new(self.name.clone())
                .description(self.description.clone())
                .kind(self.kind.clone())
                .nsfw(false)
        } else {
            let mut command = serenity::all::CreateCommand::new(self.name.clone())
                .description(self.description.clone())
                .kind(self.kind.clone())
                .nsfw(false);
            for option in &self.options {
                command = command.add_option(option.clone().to_owned());
            }
            command
        }
    }

    pub fn get_permissions(&self) -> Vec<PermissionLevel> {
        self.permissions.clone()
    }
}

pub enum CommandArguments<'a> {
    Slash(
        Option<HashMap<String, CommandDataOptionValue>>,
        &'a CommandInteraction,
    ),
    Legacy(Option<Vec<LegacyOption>>, &'a Message),
}
#[derive(Debug, Clone, Default)]
pub struct CommandResponse {
    content: Option<String>,
    embeds: Vec<CreateEmbed>,
    components: Vec<CreateActionRow>,
    attachments: Vec<CreateAttachment>,
    poll: Option<CreatePoll<Ready>>,
    ephemeral: bool,
    reply: bool,
}

impl CommandResponse {
    pub fn new(content: impl Into<String>, embeds: Vec<CreateEmbed>) -> Self {
        let content = content.into();
        Self {
            content: if content.is_empty() {
                None
            } else {
                Some(content)
            },
            embeds,
            ..Default::default()
        }
    }

    pub fn new_content(content: impl Into<String>) -> Self {
        let content = content.into();
        if content.len() > 2000 {
            error!("Content length exceeds 2000 characters");
            process::exit(1);
        }
        Self {
            content: if content.is_empty() {
                None
            } else {
                Some(content)
            },
            ..Default::default()
        }
    }

    pub fn new_embeds(embeds: Vec<CreateEmbed>) -> Self {
        Self {
            embeds,
            ..Default::default()
        }
    }

    pub fn new_components(components: Vec<CreateActionRow>) -> Self {
        Self {
            components,
            ..Default::default()
        }
    }

    pub fn new_attachments(attachments: Vec<CreateAttachment>) -> Self {
        Self {
            attachments,
            ..Default::default()
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        let content = content.into();
        if content.len() > 2000 {
            error!("Content length exceeds 2000 characters");
            process::exit(1);
        }
        self.content = Some(content);
        self
    }

    pub fn embeds(mut self, embeds: Vec<CreateEmbed>) -> Self {
        self.embeds = embeds;
        self
    }

    pub fn components(mut self, components: Vec<CreateActionRow>) -> Self {
        self.components = components;
        self
    }

    pub fn add_attachment(mut self, attachment: CreateAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    pub fn add_embed(mut self, embed: CreateEmbed) -> Self {
        self.embeds.push(embed);
        self
    }

    pub fn poll(mut self, poll: CreatePoll<Ready>) -> Self {
        self.poll = Some(poll);
        self
    }

    pub fn ephemeral(mut self) -> Self {
        self.ephemeral = true;
        self
    }

    pub fn reply(mut self) -> Self {
        self.reply = true;
        self
    }

    pub fn should_reply(&self) -> bool {
        self.reply
    }

    pub fn to_msg(&self) -> CreateMessage {
        let mut msg = CreateMessage::new();
        if let Some(ref content) = self.content {
            msg = msg.content(content);
        }
        if !self.embeds.is_empty() {
            msg = msg.embeds(self.embeds.clone());
        }
        if !self.components.is_empty() {
            msg = msg.components(self.components.clone());
        }
        if !self.attachments.is_empty() {
            msg = msg.add_files(self.attachments.clone());
        }
        if let Some(ref poll) = self.poll {
            msg = msg.poll(poll.clone());
        }
        msg
    }

    pub fn to_interaction_msg(&self) -> CreateInteractionResponseMessage {
        let mut msg = CreateInteractionResponseMessage::new();
        if let Some(ref content) = self.content {
            msg = msg.content(content);
        }
        if !self.embeds.is_empty() {
            msg = msg.embeds(self.embeds.clone());
        }
        if !self.components.is_empty() {
            msg = msg.components(self.components.clone());
        }
        if !self.attachments.is_empty() {
            msg = msg.add_files(self.attachments.clone());
        }
        if let Some(ref poll) = self.poll {
            msg = msg.poll(poll.clone());
        }
        msg
    }
}
#[derive(Debug, Clone)]
pub enum LegacyOption {
    Member(Member),
    Channel(GuildChannel),
    Role(Role),
    Text(String),
    Time(Duration),
    Integer(i64),
    Boolean(bool),
}

pub enum UserType {
    User(User),
    Member(Member),
}

impl LegacyOption {
    pub fn parse(content: &str, location: Option<(Guild, GuildChannel)>) -> Vec<Self> {
        let mut options = Vec::new();
        let mut text: Option<String> = Option::None;
        for part in content.split(' ') {
            if part.is_empty() {
                continue;
            }
            if text.is_none() {
                if part == "true" || part == "yes" {
                    options.push(LegacyOption::Boolean(true));
                    continue;
                } else if part == "false" || part == "no" {
                    options.push(LegacyOption::Boolean(false));
                    continue;
                }

                if let Ok(num) = part.parse::<i64>() {
                    options.push(LegacyOption::Integer(num));
                    continue;
                }

                if let Some(time) = Self::parse_time(part) {
                    options.push(LegacyOption::Time(time));
                    continue;
                }

                if (part.starts_with("<@") || part.starts_with("<#")) && part.ends_with('>') {
                    let id_str = &part[2..part.len() - 1];
                    if let Some(id_str) = id_str.strip_prefix("&") {
                        if let Ok(id) = id_str.parse::<u64>() && let Some((ref guild, _)) = location {
                            let Some(role) = guild.roles.get(&RoleId::new(id)) {
                                options.push(LegacyOption::Role(role.id));
                                continue;
                            } else {
                                println!("Role ID not found in guild: {}", id);
                            }
                            options.push(LegacyOption::Role(serenity::all::RoleId::new(id)));
                        } else {
                            println!("Invalid ID format in legacy role option: {}", id_str);
                        }
                    } else if let Ok(id) = id_str.parse::<u64>()
                        && let Some((ref guild, _)) = location
                    {
                        if part.starts_with("<@")
                            && let Some(member) = guild.members.get(&UserId::new(id))
                        {
                            options.push(LegacyOption::Member(member.clone()));
                        } else if part.starts_with("<#")
                            && let Some(channel) = guild.channels.get(&ChannelId::new(id))
                        {
                            options.push(LegacyOption::Channel(channel.clone()));
                        }
                    } else {
                        println!("Invalid ID format in legacy option: {}", part);
                    }
                    continue;
                }
            }
            // If it doesn't match any known type, treat it as a text option.
            if let Some(ref mut t) = text {
                if !t.is_empty() {
                    t.push(' ');
                }
                t.push_str(part);
            } else {
                text = Some(part.to_string());
            }
        }
        if let Some(text) = text {
            options.push(LegacyOption::Text(text));
        }

        options
    }

    fn parse_time(arg: &str) -> Option<Duration> {
        // This function parses a time string like "1h30m" into a Duration object.
        let mut total_seconds = 0;
        let mut current_number = String::new();
        for c in arg.chars() {
            if c.is_ascii_digit() {
                current_number.push(c);
            } else if !current_number.is_empty() {
                let value = current_number.parse::<i64>().ok()?;
                match c {
                    'y' => total_seconds += value * 60 * 60 * 24 * 365, // years
                    'w' => total_seconds += value * 60 * 60 * 24 * 7,   // weeks
                    'd' => total_seconds += value * 60 * 60 * 24,       // days
                    'h' => total_seconds += value * 60 * 60,            // hours
                    'm' => total_seconds += value * 60,
                    's' => total_seconds += value,
                    _ => return None, // Invalid character
                }
                current_number.clear();
            } else {
                return None; // Invalid format
            }
        }

        if !current_number.is_empty() {
            let value = current_number.parse::<i64>().ok()?;
            total_seconds += value; // Add any remaining seconds
        }
        if total_seconds > 0 {
            Some(Duration::seconds(total_seconds))
        } else {
            None // No valid time parsed
        }
    }

    pub fn time_str(duration: &TimeDelta) -> String {
        let mut secs = duration.num_seconds();

        if secs == 0 {
            return "0s".into();
        }

        let weeks = secs / (7 * 24 * 3600);
        secs %= 7 * 24 * 3600;

        let days = secs / (24 * 3600);
        secs %= 24 * 3600;

        let hours = secs / 3600;
        secs %= 3600;

        let minutes = secs / 60;
        secs %= 60;

        let mut parts = Vec::new();
        if weeks > 0 {
            parts.push(format!(
                "{} week{}",
                weeks,
                if weeks == 1 { "" } else { "s" }
            ));
        }
        if days > 0 {
            parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
        }
        if hours > 0 {
            parts.push(format!(
                "{} hour{}",
                hours,
                if hours == 1 { "" } else { "s" }
            ));
        }
        if minutes > 0 {
            parts.push(format!(
                "{} minute{}",
                minutes,
                if minutes == 1 { "" } else { "s" }
            ));
        }
        if secs > 0 {
            parts.push(format!(
                "{} second{}",
                secs,
                if secs == 1 { "" } else { "s" }
            ));
        }

        parts.join(" ")
    }
}

impl Display for LegacyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LegacyOption::Member(m) => write!(f, "<@{}>", m.user.id.get()),
            LegacyOption::Channel(c) => write!(f, "<#{}>", c.id.get()),
            LegacyOption::Role(id) => write!(f, "<@&{}>", id.get()),
            LegacyOption::Text(text) => write!(f, "{}", text),
            LegacyOption::Time(duration) => write!(f, "{}", Self::time_str(duration)),
            LegacyOption::Integer(num) => write!(f, "{}", num),
            LegacyOption::Boolean(b) => write!(f, "{}", b),
        }
    }
}

pub type CommandTemplate = (ICommand, Arc<dyn CommandTrait>);
pub type AutocompleteResponse = HashMap<String, Value>;
