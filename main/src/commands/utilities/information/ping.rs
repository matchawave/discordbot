use std::sync::Arc;

use chrono::Utc;
use serenity::{
    all::{
        CacheHttp, CommandType, Context, CreateInteractionResponse,
        CreateInteractionResponseMessage, EditInteractionResponse, EditMessage, Guild,
        GuildChannel,
    },
    async_trait,
};
use utils::{
    CommandArguments, CommandResponse, CommandTemplate, CommandTrait, ICommand, UserType, error,
};

use crate::{ElapsedTime, ShardManagerContainer};

const COMMAND_NAME: &str = "ping";
const COMMAND_DESCRIPTION: &str = "Ping the bot to check latency";

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
    fn is_slash(&self) -> bool {
        true
    }
    fn is_legacy(&self) -> bool {
        true
    }
    async fn execute<'a>(
        &self,
        ctx: &'a Context,
        _: UserType,
        _: Option<(Guild, GuildChannel)>,
        args: CommandArguments<'a>,
    ) -> Result<Option<CommandResponse>, String> {
        let shard_manager = {
            let data = ctx.data.read().await;
            let Some(data) = data.get::<ShardManagerContainer>().cloned() else {
                return Err("Shard manager not found".into());
            };
            data
        };

        let latency = {
            let runner = shard_manager.runners.lock().await;
            runner.get(&ctx.shard_id).and_then(|info| info.latency)
        };

        let current_time = Utc::now();
        let mut response_message = String::from("It took {time} to ping.");
        let http = ctx.http();
        let created_at = match args {
            CommandArguments::Slash(_, interaction) => interaction.id.created_at().to_utc(),
            CommandArguments::Legacy(_, message) => message.timestamp.to_utc(),
        };
        let elapsed = latency
            .map(|latency| latency.as_millis())
            .unwrap_or((current_time - created_at).abs().num_milliseconds() as u128);
        response_message = response_message.replace("{time}", format!("{}ms", elapsed).as_str());

        let edit_timer = ElapsedTime::new();
        let new_msg = match args {
            CommandArguments::Slash(_, interaction) => {
                let response =
                    CreateInteractionResponseMessage::new().content(response_message.clone());
                interaction
                    .create_response(http, CreateInteractionResponse::Message(response))
                    .await
                    .ok();

                None
            }
            CommandArguments::Legacy(_, message) => {
                message.reply(http, response_message.clone()).await.ok()
            }
        };
        response_message += format!(" (edit: {}ms)", edit_timer.elapsed_ms()).as_str();

        match args {
            CommandArguments::Slash(_, interaction) => {
                if let Err(e) = interaction
                    .edit_response(
                        http,
                        EditInteractionResponse::new().content(response_message),
                    )
                    .await
                {
                    error!("Failed to edit interaction response: {}", e);
                }
            }
            CommandArguments::Legacy(_, _) => {
                if let Some(mut m) = new_msg
                    && let Err(e) = m
                        .edit(http, EditMessage::new().content(response_message))
                        .await
                {
                    error!("Failed to edit message: {}", e);
                }
            }
        }

        Ok(None)
    }
}
