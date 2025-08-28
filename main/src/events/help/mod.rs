use serenity::all::{CacheHttp, Context, CreateMessage, Message};
use utils::error;

use crate::{ServerPrefix, ServerPrefixes};

pub async fn is_asking_for_bot_prefix(ctx: &Context, msg: &Message) -> bool {
    let bot_id = ctx.cache.current_user().id.get();
    let Some(guild_id) = msg.guild_id else {
        return false;
    };
    let data = ctx.data.read().await;
    if msg.content == format!("<@{}>", bot_id) || msg.content == format!("<@!{}>", bot_id) {
        let prefix_repo = data
            .get::<ServerPrefixes>()
            .expect("Expected ServerPrefixes in TypeMap");
        let prefixes = prefix_repo.read().await;
        let prefix = prefixes
            .get(&ServerPrefix::Guild(guild_id))
            .or_else(|| prefixes.get(&ServerPrefix::Default))
            .expect("Default prefix must be set");

        let new_msg = format!("My prefix in this server is `{}`.", prefix);
        let message = CreateMessage::new().content(new_msg).reference_message(msg);

        if let Err(e) = msg.channel_id.send_message(ctx.http(), message).await {
            error!("Failed to send AFK mention reply: {}", e);
        };
        return true;
    }
    false
}
