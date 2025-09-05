use serenity::all::{CacheHttp, Context, Message};
use utils::{CommandArguments, LegacyOption, UserType, error, info, warning};

use crate::{Commands, ElapsedTime, ServerPrefix, ServerPrefixes};

pub async fn is_command(ctx: &Context, msg: &Message) -> bool {
    let timer = ElapsedTime::new();
    if msg.author.bot {
        return false;
    }

    let Some(guild_id) = msg.guild_id else {
        return false;
    };

    let location = {
        let guild = guild_id.to_guild_cached(&ctx.cache).map(|g| g.clone());
        guild.and_then(|g| {
            let channel = g.channels.get(&msg.channel_id).cloned()?;
            Some((g, channel))
        })
    };

    let member = match location {
        Some((ref guild, _)) => match guild.members.get(&msg.author.id) {
            Some(m) => UserType::Member(m.clone()),
            None => match guild.member(&ctx.http, msg.author.id).await {
                Ok(m) => UserType::Member(m.clone().into_owned()),
                Err(_) => UserType::User(msg.author.clone()),
            },
        },
        None => UserType::User(msg.author.clone()),
    };

    let prefix = {
        let data = ctx.data.read().await;
        let p = data
            .get::<ServerPrefixes>()
            .expect("ServerPrefixes not initialized")
            .read()
            .await;

        p.get(&ServerPrefix::Guild(guild_id)).cloned().unwrap_or(
            p.get(&ServerPrefix::Default)
                .expect("Default prefix not set")
                .clone(),
        )
    };

    if !msg.content.starts_with(&prefix) {
        info!(
            "Message does not start with prefix '{}' ({}ms)",
            prefix,
            timer.elapsed_ms()
        );
        return false;
    }

    let commands = {
        let data = ctx.data.read().await;
        data.get::<Commands>()
            .expect("Commands not initialized")
            .clone()
    };

    let mut content = msg.content.trim_start_matches(&prefix);
    let c_name = content.split_whitespace().next().unwrap_or("");
    content = content.trim_start_matches(c_name).trim();

    let Some((c, _perms)) = commands.get(c_name) else {
        warning!("Command '{}' not found", c_name);
        return false;
    };

    if !c.is_legacy() {
        warning!("Command '{}' is not a legacy command", c_name);
        return false;
    }

    let options = LegacyOption::parse(content);

    let args = CommandArguments::Legacy(Some(options), msg);

    match c.execute(ctx, member, location.clone(), args).await {
        Ok(r) => {
            if let Some(msg_response) = r {
                let mut new_msg = msg_response.to_msg();
                if msg_response.should_reply() {
                    new_msg = new_msg.reference_message(msg);
                }
                if let Some((_, channel)) = location
                    && let Err(e) = channel.send_message(ctx.http(), new_msg).await
                {
                    error!("Failed to send message: {}", e);
                    return false;
                }
            }
            info!("Executed command '{}' ({}ms)", c_name, timer.elapsed_ms());
            true
        }
        Err(e) => {
            error!("Error executing command '{}': {}", c_name, e);
            false
        }
    }
}
