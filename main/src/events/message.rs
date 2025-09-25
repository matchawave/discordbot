use serenity::all::{
    ActionRowComponent, ButtonKind, ChannelId, Context, GuildId, Message, MessageId,
    MessageUpdateEvent,
};
use utils::parse_button_id;

use crate::{Paginations, extras, handler::commands, snipes, user_afk};

pub async fn create(ctx: Context, message: Message) {
    let Some(guild_id) = message.guild_id else {
        return;
    };
    if message.author.bot {
        let current_bot = ctx.cache.current_user().clone();
        if message.author.id == current_bot.id {
            organize_components(&ctx, &message).await;
        }
        return;
    }

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

    let message = {
        let mut msg = Message::default();
        env.apply_to_message(&mut msg);
        msg
    };

    if message.author.bot {
        return;
    }

    let old_message = ctx
        .cache
        .message(message.channel_id, message.id)
        .map(|m| m.clone());

    snipes::edit(&ctx.data, &message, &guild_id).await;

    if extras::is_asking_for_bot_prefix(&ctx, &message).await {
        return;
    }
    if commands::message::is_command(&ctx, &message).await {
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

    let Some(message) = ctx.cache.message(channel_id, message_id).map(|m| m.clone()) else {
        return;
    };

    if message.author.bot {
        return;
    }

    snipes::delete(&ctx.data, &message, &guild_id).await;
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

async fn organize_components(ctx: &Context, message: &Message) {
    let components = message.components.clone();

    for row in components {
        if let Some(first) = row.components.first() {
            match first {
                ActionRowComponent::Button(button) => {
                    if let ButtonKind::NonLink { custom_id, style } = &button.data
                        && let Some((section, id, userid, action)) =
                            parse_button_id(custom_id.as_str())
                    {
                        match section {
                            "page" => {
                                let pages = {
                                    let data = ctx.data.read().await;
                                    data.get::<Paginations>()
                                        .expect("Failed to get paginations")
                                        .clone()
                                };

                                if let Some(page) = pages.get(&id).await {
                                    let mut page = page.write().await;
                                    page.set_id(message.channel_id, message.id);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                ActionRowComponent::SelectMenu(select) => {
                    // Handle select menu component
                }
                ActionRowComponent::InputText(input) => {
                    // Handle input text component
                }
                _ => {}
            }
        }
    }
}
