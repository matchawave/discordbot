use std::sync::Arc;

use serenity::{
    all::{
        Cache, CacheHttp, ChannelId, Context, Guild, GuildChannel, GuildId, Http, Member, Message,
        UserId,
    },
    model::guild,
};
use utils::{
    BotPermission, CommandArguments, CommandTrait, Data, LegacyOption, UserType, error, info,
    warning,
};

use crate::{Commands, ElapsedTime, ServerPrefix, ServerPrefixes};

pub async fn is_command(ctx: &Context, msg: &Message) -> bool {
    let timer = ElapsedTime::new();
    let data = ctx.data.clone();

    let Some(prefix) = get_prefix(&data, msg).await else {
        return false;
    };

    let Some((c_name, command, permissions, content)) = get_command(&data, msg, prefix).await
    else {
        return false;
    };

    let location = get_location(&ctx.cache, msg.guild_id, msg.channel_id).await;
    let user = msg.author.clone();
    let member = location
        .as_ref()
        .and_then(|(g, _)| g.members.get(&user.id).cloned().map(UserType::Member))
        .unwrap_or(UserType::User(user.clone()));

    let options = LegacyOption::parse(&content, &location);

    let args = CommandArguments::Legacy(Some(options), msg);

    match command.execute(ctx, member, location.clone(), args).await {
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

pub async fn get_prefix(data: &Data, msg: &Message) -> Option<String> {
    if msg.author.bot {
        return None;
    }

    let guild_id = msg.guild_id?;

    let data = data.read().await;
    let p = data
        .get::<ServerPrefixes>()
        .expect("ServerPrefixes not initialized")
        .read()
        .await;

    p.get(&ServerPrefix::Guild(guild_id))
        .cloned()
        .or(p.get(&ServerPrefix::Default).cloned())
}

pub async fn get_command(
    data: &Data,
    msg: &Message,
    prefix: String,
) -> Option<(String, Arc<dyn CommandTrait>, Vec<BotPermission>, String)> {
    if !msg.content.starts_with(&prefix) {
        return None;
    }

    let content = msg.content.trim_start_matches(&prefix);
    let c_name = content.split_whitespace().next().unwrap_or("");

    let data = data.read().await;
    let commands = data
        .get::<Commands>()
        .expect("Commands not initialized")
        .clone();

    match commands.get(c_name).cloned() {
        Some((c, perms)) if c.is_legacy() => Some((
            c_name.to_string(),
            c,
            perms,
            content.trim_start_matches(c_name).trim().to_string(),
        )),
        _ => None,
    }
}

pub async fn get_location(
    cache: &Arc<Cache>,
    guild_id: Option<GuildId>,
    channel_id: ChannelId,
) -> Option<(Guild, GuildChannel)> {
    let guild = guild_id.and_then(|id| cache.guild(id).map(|g| g.clone()));

    guild.and_then(|g| {
        g.channels
            .get(&channel_id)
            .cloned()
            .map(|channel| (g, channel))
    })
}
