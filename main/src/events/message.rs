use serenity::all::{
    ChannelId, Context, EventHandler, GuildId, Message, MessageId, MessageUpdateEvent,
};

use crate::{extras, handler::commands, user_afk};

pub async fn create(ctx: Context, message: Message) {
    let Some(guild_id) = message.guild_id else {
        return;
    };
    tokio::spawn(user_afk::notify_afk_mentions(ctx.clone(), message.clone()));
    user_afk::check_afk_status(&ctx, &message).await;
    if extras::is_asking_for_bot_prefix(&ctx, &message).await {
        return;
    }
    if commands::message::is_command(&ctx, &message).await {
        return;
    }
}

pub async fn update(ctx: Context, env: MessageUpdateEvent) {
    let Some(guild_id) = env.guild_id else {
        return;
    };
    let new_message = {
        let mut msg = Message::default();
        env.apply_to_message(&mut msg);
        msg
    };
    let old_message = ctx
        .cache
        .message(new_message.channel_id, new_message.id)
        .map(|m| m.clone());

    if extras::is_asking_for_bot_prefix(&ctx, &new_message).await {
        return;
    }
    if commands::message::is_command(&ctx, &new_message).await {
        return;
    }
}

pub async fn delete(
    ctx: Context,
    channel_id: ChannelId,
    message_id: MessageId,
    guild_id: Option<GuildId>,
) {
    let Some(guild_id) = guild_id else {
        return;
    };
}

pub async fn bulk_delete(
    ctx: Context,
    channel_id: ChannelId,
    guild_id: Option<GuildId>,
    ids: Vec<MessageId>,
) {
    let Some(guild_id) = guild_id else {
        return;
    };
}
