use std::sync::Arc;

use serenity::all::{
    CacheHttp, ChannelId, CommandDataOptionValue, Context, Guild, GuildChannel, Http, Member,
    PartialGuild, User, UserId,
};
use utils::{BotPermission, CommandArguments, CommandTrait, LegacyOption, error};

mod example;

pub mod configuration;
pub mod fun;
pub mod integration;
pub mod security;
pub mod server;
pub mod utilities;

pub type CommandModule = (Arc<dyn CommandTrait>, Vec<BotPermission>);

async fn command_user_target<'a>(ctx: &'a Context, args: &'a CommandArguments<'a>) -> Option<User> {
    let args = args.clone();
    match args {
        CommandArguments::Legacy(args, _) => args.and_then(|arg| {
            arg.first().and_then(|opt| match opt {
                LegacyOption::Member(m) => Some(m.user.clone()),
                _ => None,
            })
        }),
        CommandArguments::Slash(arg, _) => {
            let user_id = arg.and_then(|a| a.get("user").cloned());
            user_interaction_option(ctx, user_id).await
        }
    }
}
async fn command_member_target<'a>(
    ctx: &'a Context,
    args: &'a CommandArguments<'a>,
    guild: &'a Guild,
) -> Option<Member> {
    match args.clone() {
        CommandArguments::Legacy(args, _) => args.and_then(|arg| {
            arg.first().and_then(|opt| match opt {
                LegacyOption::Member(m) => Some(m.clone()),
                _ => None,
            })
        }),
        CommandArguments::Slash(arg, _) => {
            let user_id = arg.and_then(|a| a.get("user").cloned());
            member_interaction_option(ctx, guild, user_id).await
        }
    }
}

async fn command_channel_target<'a>(
    ctx: &'a Context,
    args: &'a CommandArguments<'a>,
    guild: &'a Guild,
) -> Option<GuildChannel> {
    match args.clone() {
        CommandArguments::Legacy(args, _) => args.and_then(|arg| {
            arg.first().and_then(|opt| match opt {
                LegacyOption::Channel(c) => Some(c.clone()),
                _ => None,
            })
        }),
        CommandArguments::Slash(arg, _) => {
            let channel_id = arg.and_then(|a| a.get("channel").cloned());
            channel_interaction_option(ctx, guild, channel_id).await
        }
    }
}

async fn user_interaction_option(
    ctx: &Context,
    user_option: Option<CommandDataOptionValue>,
) -> Option<User> {
    if let Some(CommandDataOptionValue::User(user_id)) = user_option {
        let mut user = user_id.to_user_cached(ctx).map(|u| u.clone());
        if user.is_none() {
            user = fetch_user(ctx.http.clone(), &user_id).await;
        }
        return user;
    }
    None
}

async fn member_interaction_option(
    ctx: &Context,
    guild: &Guild,
    user_option: Option<CommandDataOptionValue>,
) -> Option<Member> {
    if let Some(CommandDataOptionValue::User(user_id)) = user_option {
        let mut member = guild.members.get(&user_id).cloned();
        if member.is_none() {
            member = fetch_member(ctx.http.clone(), guild, &user_id).await;
        }
        return member;
    }
    None
}

async fn channel_interaction_option(
    ctx: &Context,
    guild: &Guild,
    channel_option: Option<CommandDataOptionValue>,
) -> Option<GuildChannel> {
    if let Some(CommandDataOptionValue::Channel(channel_id)) = channel_option {
        let mut channel = guild.channels.get(&channel_id).cloned();
        if channel.is_none() {
            channel = fetch_channel(ctx.http.as_ref(), guild, &channel_id).await;
        }
        return channel;
    }
    None
}

async fn fetch_user(http: impl CacheHttp, id: &UserId) -> Option<User> {
    match id.to_user(http).await {
        Ok(user) => Some(user),
        Err(e) => {
            error!("Failed to fetch user with ID {}: {}", id, e);
            None
        }
    }
}

async fn fetch_member(http: impl CacheHttp, guild: &Guild, user_id: &UserId) -> Option<Member> {
    match guild.member(http, user_id).await {
        Ok(member) => Some(member.clone().into_owned()),
        Err(e) => {
            error!(
                "Failed to fetch member with ID {} in guild {}: {}",
                user_id, guild.id, e
            );
            None
        }
    }
}

async fn fetch_guild(http: impl CacheHttp, guild_id: &u64) -> Option<PartialGuild> {
    match Guild::get(http, *guild_id).await {
        Ok(guild) => Some(guild),
        Err(e) => {
            error!("Failed to fetch guild with ID {}: {}", guild_id, e);
            None
        }
    }
}

async fn fetch_channel(
    http: &impl AsRef<Http>,
    guild: &Guild,
    channel_id: &ChannelId,
) -> Option<GuildChannel> {
    match guild.channels(http).await {
        Ok(channels) => channels.get(channel_id).cloned(),
        Err(e) => {
            error!(
                "Failed to fetch channel with ID {} in guild {}: {}",
                channel_id, guild.id, e
            );
            None
        }
    }
}

// async fn fetch_member_roles(http: &impl CacheHttp, guild: &Guild, member: &Member) -> Option<Vec<serenity::all::Role>> {
//     let mut roles = Vec::new();

//     if let Some(updated_guild) = fetch_guild(http, &guild.id).await {}

//     for role_id in &member.roles {
//         match guild.roles.get(role_id) {
//             Some(role) => roles.push(role.clone()),
//             None => {
//                 // If the role is not in the cache, try to fetch the guild again to update the cache
//                 if let Some(updated_guild) = fetch_guild(http, &guild.id.0).await {
//                     if let Some(fetched_role) = updated_guild.roles.get(role_id) {
//                         roles.push(fetched_role.clone());
//                     } else {
//                         error!("Role with ID {} not found in guild {}", role_id, guild.id);
//                     }
//                 } else {
//                     error!("Failed to update guild cache for guild {}", guild.id);
//                 }
//             }
//         }
//     }
//     Some(roles)
// }
