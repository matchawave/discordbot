use serenity::all::{CacheHttp, Context, CreateMessage, Mentionable, Message};
use utils::error;

use crate::{ServerPrefix, ServerPrefixes};

pub async fn is_asking_for_bot_prefix(ctx: &Context, msg: &Message) -> bool {
    let bot_id = ctx.cache.current_user().id;
    let Some(guild_id) = msg.guild_id else {
        return false;
    };

    let data = ctx.data.clone();
    if msg.content == bot_id.mention().to_string() {
        let repo = {
            let data_read = data.read().await;
            data_read
                .get::<ServerPrefixes>()
                .cloned()
                .expect("Expected ServerPrefixes in TypeMap")
        };

        let prefix = repo
            .get(&ServerPrefix::Guild(guild_id))
            .or(repo.get(&ServerPrefix::Default))
            .expect("Default prefix must be set")
            .clone();

        let new_msg = format!("My prefix in this server is `{}`.", prefix);
        let message = CreateMessage::new().content(new_msg).reference_message(msg);

        if let Err(e) = msg.channel_id.send_message(ctx.http(), message).await {
            error!("Failed to send AFK mention reply: {}", e);
        };
        return true;
    }
    false
}
