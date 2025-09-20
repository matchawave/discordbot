use std::sync::Arc;

use chrono::Utc;
use serenity::{
    all::{
        CacheHttp, Context, CreateMessage, FormattedTimestampStyle, GuildId, Http, Mentionable,
        Message, Timestamp, User, UserId,
    },
    utils::FormattedTimestamp,
};
use tokio::sync::RwLock;
use utils::{LegacyOption, UserConfigHash, error, warning};

use crate::{UserAFK, UserAFKData};

pub async fn check_afk_status(ctx: &Context, msg: &Message) {
    let Some(guild_id) = msg.guild_id else {
        return;
    };

    let user = &msg.author;
    let data = ctx.data.clone();
    let afk_repo = {
        let data = data.read().await;
        data.get::<UserAFK>()
            .cloned()
            .expect("UserAFK data not found")
    };

    let Some(status) = ({
        // Check AFK status of user
        let afk_data = afk_repo.read().await;
        afk_data.get_raw(&guild_id, &user.id).map(|s| s.clone())
    }) else {
        return;
    };

    let current_time = Utc::now();
    let elapsed_time = current_time - status.last_active;

    let reply_message = format!(
        "Welcome back <@{}>, you were away for {}",
        user.id,
        LegacyOption::time_str(&elapsed_time)
    );

    if let Err(e) = msg.reply(ctx.http(), reply_message).await {
        error!("Failed to send AFK reply: {}", e);
    };

    let mut repo = afk_repo.write().await;
    repo.remove(&guild_id, &user.id);
}

pub async fn notify_afk_mentions(ctx: Context, msg: Message) {
    let data = ctx.data.clone();
    let afk_repo = {
        let data = data.read().await;
        data.get::<UserAFK>()
            .cloned()
            .expect("UserAFK data not found")
    };

    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return, // Not in a guild
    };

    if let Some(ref_msg) = &msg.referenced_message {
        if ref_msg.author.id == msg.author.id {
            return; // Ignore self-replies
        }
        handle_message(ctx.http(), &afk_repo, &ref_msg.author.id, &guild_id, &msg).await;
    }

    for user in &msg.mentions {
        if user.id == msg.author.id {
            continue; // Ignore self-mentions
        }
        handle_message(ctx.http(), &afk_repo, &user.id, &guild_id, &msg).await;
    }
}

async fn handle_message(
    http: &Http,
    repo: &Arc<RwLock<UserConfigHash<UserAFKData>>>,
    user_id: &UserId,
    guild_id: &GuildId,
    msg: &Message,
) {
    let Some(status) = ({
        // Check AFK status of user
        let afk_data = repo.read().await;
        afk_data.get_raw(guild_id, user_id).map(|s| s.clone())
    }) else {
        return;
    };

    let Ok(timestamp) = Timestamp::from_millis(status.last_active.timestamp_millis())
        .map(|ts| FormattedTimestamp::new(ts, Some(FormattedTimestampStyle::RelativeTime)))
    else {
        warning!("Failed to parse AFK timestamp");
        return;
    };

    let reply_message = format!(
        "{} is AFK: {} - {}",
        user_id.mention(),
        status.afk_status,
        timestamp
    );

    let message = CreateMessage::new()
        .content(reply_message)
        .reference_message(msg);

    if let Err(e) = msg.channel_id.send_message(http, message).await {
        error!("Failed to send AFK mention reply: {}", e);
    };
}
