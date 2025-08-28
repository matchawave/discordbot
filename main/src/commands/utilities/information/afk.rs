use std::sync::Arc;

use serenity::{
    all::{ChannelId, CommandType, Context, GuildId, User},
    async_trait,
};

use utils::{CommandArguments, CommandResponse, CommandTemplate, CommandTrait, ICommand};

use crate::{AFKAccess, UserAFK, UserAFKData};

const COMMAND_NAME: &str = "afk";
const COMMAND_DESCRIPTION: &str = "Set your AFK status";

fn get_status<'a>(command: &CommandArguments<'a>) -> Option<String> {
    match command {
        CommandArguments::Slash(options, _) => options.clone().and_then(|m| {
            m.get("status")
                .and_then(|v| v.as_str().map(|s| s.to_string()))
        }),
        CommandArguments::Legacy(options, _) => options.clone().and_then(|opts| {
            let output = opts
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            if output.is_empty() {
                None
            } else {
                Some(output)
            }
        }),
    }
}

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
    fn is_legacy(&self) -> bool {
        true
    }
    fn is_slash(&self) -> bool {
        true
    }
    async fn execute<'a>(
        &self,
        ctx: &'a Context,
        user: &'a User,
        guild_and_channel: Option<(GuildId, ChannelId)>,
        args: CommandArguments<'a>,
    ) -> Option<CommandResponse> {
        let (guild_id, _) = guild_and_channel?;

        let status = get_status(&args).map(UserAFKData::new).unwrap_or_default();

        let afk_repo = {
            let data = ctx.data.read().await;
            data.get::<UserAFK>()
                .expect("UserAFK data not found")
                .clone()
        };

        {
            let repo = afk_repo.read().await;
            match repo.get(&AFKAccess::Guild(guild_id, user.id)) {
                Some(_) => {
                    return Some(
                        CommandResponse::new_content(
                            "You are already marked as AFK in this server.".to_string(),
                        )
                        .reply(),
                    );
                } // User has server specific AFK status
                None => {
                    if afk_repo
                        .read()
                        .await
                        .get(&AFKAccess::User(user.id))
                        .is_some()
                    {
                        return Some(
                            CommandResponse::new_content(
                                "You are already marked as AFK globally.".to_string(),
                            )
                            .reply(),
                        );
                    }
                }
            }
        }

        let mut repo = afk_repo.write().await;
        repo.insert(crate::AFKAccess::User(user.id), status.clone());

        Some(
            CommandResponse::new_content(format!(
                "You are now AFK with the status: {}",
                status.afk_status
            ))
            .reply(),
        )
    }
}
