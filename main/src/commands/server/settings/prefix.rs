use std::sync::Arc;

use serenity::{
    all::{
        CommandDataOptionValue, CommandType, Context, CreateCommandOption, Guild, GuildChannel,
        GuildId,
    },
    async_trait,
};

use utils::{
    BotPermission, CommandArguments, CommandResponse, CommandTemplate, CommandTrait, Data,
    ICommand, LegacyOption, UserType, error,
};

use crate::{ServerPrefix, ServerPrefixes};

const COMMAND_NAME: &str = "prefix";
const COMMAND_DESCRIPTION: &str = "Server prefix.";

pub struct Command;

pub fn command() -> CommandTemplate {
    let set_options = CreateCommandOption::new(
        serenity::all::CommandOptionType::SubCommand,
        "set",
        "Set the server prefix",
    )
    .add_sub_option(
        CreateCommandOption::new(
            serenity::all::CommandOptionType::String,
            "value",
            "The new prefix",
        )
        .required(true),
    );

    let remove_options = CreateCommandOption::new(
        serenity::all::CommandOptionType::SubCommand,
        "remove",
        "Remove the custom server prefix",
    );

    let get_options = CreateCommandOption::new(
        serenity::all::CommandOptionType::SubCommand,
        "get",
        "Get the current server prefix",
    );

    (
        ICommand::new(
            COMMAND_NAME.to_string(),
            COMMAND_DESCRIPTION.to_string(),
            CommandType::ChatInput,
            vec![set_options, remove_options, get_options],
            vec![BotPermission::BotMaster],
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
        location: Option<(Guild, GuildChannel)>,
        args: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String> {
        let Some((guild, _)) = location else {
            error!("This command can only be used in a server");
            return Err("This command can only be used in a server.".into());
        };
        let data = ctx.data.clone();
        let res = match args {
            CommandArguments::Slash(Some(options), _) => {
                let options = options
                    .iter()
                    .map(|opt| (opt.0.as_str(), opt.1))
                    .collect::<Vec<_>>();
                match options.first() {
                    Some(("set", CommandDataOptionValue::SubCommand(sub_cmd))) => {
                        let CommandDataOptionValue::String(value) = sub_cmd[0].clone().value else {
                            error!("Expected string value for prefix");
                            panic!()
                        };
                        set(data, value, guild.id).await
                    }
                    Some(("remove", _)) => remove(data, guild.id).await,
                    Some(("get", _)) => get(data, guild.id).await,
                    _ => {
                        error!("Unexpected command arguments");
                        panic!()
                    }
                }
            }
            CommandArguments::Legacy(options, _) => {
                let Some(options) = options else {
                    return Ok(Some(get(data, guild.id).await));
                };

                match options.first() {
                    Some(LegacyOption::Text(value)) if value == "remove" => {
                        remove(data, guild.id).await
                    }
                    Some(LegacyOption::Text(value)) => set(data, value.clone(), guild.id).await,
                    _ => get(data, guild.id).await,
                }
            }
            _ => {
                error!("Unexpected command arguments");
                panic!()
            }
        };
        Ok(Some(res.reply()))
    }
    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
}

async fn set(data: Data, value: String, guild_id: GuildId) -> CommandResponse {
    let data = data.read().await;

    let prefix_repo = data
        .get::<ServerPrefixes>()
        .expect("Expected ServerPrefixes in TypeMap");

    let default_prefix = {
        let prefixes = prefix_repo.read().await;
        prefixes
            .get(&ServerPrefix::Default)
            .cloned()
            .expect("Default prefix must be set")
    };
    let previous_prefix = {
        let prefixes = prefix_repo.read().await;
        prefixes
            .get(&ServerPrefix::Guild(guild_id))
            .cloned()
            .unwrap_or_else(|| default_prefix.clone())
    };

    if value == default_prefix {
        let mut prefixes = prefix_repo.write().await;
        prefixes.remove(&ServerPrefix::Guild(guild_id));

        CommandResponse::new_content(format!(
            "Server prefix reset to default: `{}`",
            default_prefix
        ))
    } else if value == previous_prefix {
        CommandResponse::new_content(format!(
            "Server prefix is already set to: `{}`",
            previous_prefix
        ))
    } else {
        let mut prefixes = prefix_repo.write().await;
        prefixes.insert(ServerPrefix::Guild(guild_id), value.clone());
        CommandResponse::new_content(format!("Server prefix set to: `{}`", value))
    }
}

async fn remove(data: Data, guild_id: GuildId) -> CommandResponse {
    let data = data.read().await;
    let prefix_repo = data
        .get::<ServerPrefixes>()
        .expect("Expected ServerPrefixes in TypeMap");

    let default_prefix = {
        let prefixes = prefix_repo.read().await;
        prefixes
            .get(&ServerPrefix::Default)
            .cloned()
            .expect("Default prefix must be set")
    };
    let mut prefixes = prefix_repo.write().await;
    if prefixes.remove(&ServerPrefix::Guild(guild_id)).is_none() {
        return CommandResponse::new_content(format!(
            "Server prefix is already the default: `{}`",
            default_prefix
        ));
    }

    CommandResponse::new_content(format!(
        "Server prefix reset to default: `{}`",
        default_prefix
    ))
}

async fn get(data: Data, guild_id: GuildId) -> CommandResponse {
    let data = data.read().await;
    let prefix_repo = data
        .get::<ServerPrefixes>()
        .expect("Expected ServerPrefixes in TypeMap");
    let prefixes = prefix_repo.read().await;
    let prefix = prefixes
        .get(&ServerPrefix::Guild(guild_id))
        .or_else(|| prefixes.get(&ServerPrefix::Default))
        .expect("Default prefix must be set");
    CommandResponse::new_content(format!("Current server prefix: `{}`", prefix))
}
