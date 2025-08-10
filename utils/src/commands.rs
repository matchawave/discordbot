use std::sync::Arc;

use chrono::Duration;
use serenity::{
    all::{CommandInteraction, Context, CreateCommandOption, GuildId, Message, RoleId, UserId},
    async_trait,
};

use crate::{InteractionCommandResult, MessageResponseResult, PermissionLevel};
pub use macros::{SlashWithAutocomplete, SlashWithLegacy, SlashWithLegacyAutocomplete};

#[derive(Debug, Clone)]
pub enum LegacyOptions {
    User(UserId),
    Channel(GuildId),
    Role(RoleId),
    Text(String),
    Time(Duration),
    Integer(i64),
    Boolean(bool),
}

impl LegacyOptions {
    pub fn parse(content: &str) -> Vec<Self> {
        let mut options = Vec::new();
        let mut text: Option<String> = Option::None;
        for part in content.split(' ') {
            if part.is_empty() {
                continue;
            }
            if text.is_none() {
                if part == "true" || part == "yes" {
                    options.push(LegacyOptions::Boolean(true));
                    continue;
                } else if part == "false" || part == "no" {
                    options.push(LegacyOptions::Boolean(false));
                    continue;
                }

                if let Ok(num) = part.parse::<i64>() {
                    options.push(LegacyOptions::Integer(num));
                    continue;
                }

                if let Some(time) = Self::parse_time(part) {
                    options.push(LegacyOptions::Time(time));
                    continue;
                }

                if (part.starts_with("<@") || part.starts_with("<#")) && part.ends_with('>') {
                    let id_str = &part[2..part.len() - 1];
                    if let Some(id_str) = id_str.strip_prefix("&") {
                        if let Ok(id) = id_str.parse::<u64>() {
                            options.push(LegacyOptions::Role(serenity::all::RoleId::new(id)));
                        } else {
                            println!("Invalid ID format in legacy role option: {}", id_str);
                        }
                    } else if let Ok(id) = id_str.parse::<u64>() {
                        if part.starts_with("<@") {
                            options.push(LegacyOptions::User(serenity::all::UserId::new(id)));
                        } else if part.starts_with("<#") {
                            options.push(LegacyOptions::Channel(serenity::all::GuildId::new(id)));
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
            options.push(LegacyOptions::Text(text));
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
}

#[async_trait]
pub trait Slash: Send + Sync {
    async fn slash(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> InteractionCommandResult;
}

#[async_trait]
pub trait Legacy: Send + Sync {
    async fn legacy(
        &self,
        ctx: &Context,
        msg: &Message,
        options: Vec<LegacyOptions>,
    ) -> MessageResponseResult;
}

#[async_trait]
pub trait Autocomplete: Send + Sync {
    async fn autocomplete(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> InteractionCommandResult;
}

#[derive(Clone)]
pub enum CommandType {
    Slash(Arc<dyn Slash>),
    Legacy(Arc<dyn Legacy>),
    Autocomplete(Arc<dyn Autocomplete>),
    SlashWithAutocomplete(Arc<dyn SlashWithAutocomplete>),
    SlashWithLegacy(Arc<dyn SlashWithLegacy>),
    SlashWithLegacyAutocomplete(Arc<dyn SlashWithLegacyAutocomplete>),
}

pub trait SlashWithAutocomplete: Slash + Autocomplete + Send + Sync {}

pub trait SlashWithLegacy: Slash + Legacy + Send + Sync {}

pub trait SlashWithLegacyAutocomplete: Autocomplete + Legacy + Slash + Send + Sync {}

pub struct ICommand<'a> {
    name: &'a str,
    description: &'a str,
    options: Vec<CreateCommandOption>,
    permissions: Vec<PermissionLevel>,
    callbacks: CommandType,
}

impl<'a> ICommand<'a> {
    pub fn new(
        name: &'a str,
        description: &'a str,
        options: Vec<CreateCommandOption>,
        permissions: Vec<PermissionLevel>,
        callbacks: CommandType,
    ) -> Self {
        if options.len() >= 25 {
            panic!("Command {} has too many options", name);
        }
        if description.len() > 100 {
            panic!("Command {} has too long description", name);
        }
        if name.len() > 32 {
            panic!("Command {} has too long name", name);
        }

        if name.is_empty() {
            panic!("Command name cannot be empty");
        }
        if description.is_empty() {
            panic!("Command description cannot be empty");
        }

        Self {
            name,
            description,
            options,
            permissions,
            callbacks,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_description(&self) -> &str {
        self.description
    }

    pub fn get_options(&self) -> &[serenity::all::CreateCommandOption] {
        &self.options
    }

    pub fn get_command(&self) -> serenity::all::CreateCommand {
        if self.options.is_empty() {
            serenity::all::CreateCommand::new(self.name.to_string()).description(self.description)
        } else {
            let mut command = serenity::all::CreateCommand::new(self.name.to_string())
                .description(self.description)
                .kind(serenity::all::CommandType::ChatInput)
                .nsfw(false);
            for option in &self.options {
                command = command.add_option(option.clone().to_owned());
            }
            command
        }
    }

    pub fn get_callbacks(&self) -> CommandType {
        self.callbacks.clone()
    }

    pub fn get_permissions(&self) -> Vec<PermissionLevel> {
        self.permissions.clone()
    }
}

pub enum CommandExecutionType<'a> {
    Command(&'a CommandInteraction),
    Legacy(&'a Message, Vec<LegacyOptions>),
}
