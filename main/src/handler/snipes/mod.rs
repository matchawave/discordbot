use serenity::all::{GuildId, Message, Reaction};
use utils::Data;

use crate::{BlacklistedSnipes, EditSnipes, ReactionSnipes, Snipes};

pub async fn delete(data: &Data, message: &Message, guild_id: &GuildId) {
    let (snipes, blacklist) = {
        let data = data.read().await;
        let snipes = data
            .get::<Snipes>()
            .cloned()
            .expect("Expected Snipes in TypeMap.");
        let blacklist = data
            .get::<BlacklistedSnipes>()
            .cloned()
            .expect("Expected Blacklisted Snipes in TypeMap.");
        (snipes, blacklist)
    };

    if let Some(blacklisted) = blacklist.get(guild_id)
        && blacklisted.contains(&message.channel_id)
    {
        return;
    }

    snipes
        .entry(message.channel_id)
        .or_default()
        .push(message.clone());
}

pub async fn edit(data: &Data, message: &Message, guild_id: &GuildId) {
    let (snipes, blacklist) = {
        let data = data.read().await;
        let snipes = data
            .get::<EditSnipes>()
            .cloned()
            .expect("Expected Edit Snipes in TypeMap.");
        let blacklist = data
            .get::<BlacklistedSnipes>()
            .cloned()
            .expect("Expected Blacklisted Snipes in TypeMap.");
        (snipes, blacklist)
    };

    if let Some(blacklisted) = blacklist.get(guild_id)
        && blacklisted.contains(&message.channel_id)
    {
        return;
    }

    snipes
        .entry(message.channel_id)
        .or_default()
        .push(message.clone());
}

pub async fn reaction(data: &Data, reaction: &Reaction, guild_id: &GuildId) {
    let (snipes, blacklist) = {
        let data = data.read().await;
        let snipes = data
            .get::<ReactionSnipes>()
            .cloned()
            .expect("Expected Snipes in TypeMap.");
        let blacklist = data
            .get::<BlacklistedSnipes>()
            .cloned()
            .expect("Expected Blacklisted Snipes in TypeMap.");
        (snipes, blacklist)
    };

    if let Some(blacklisted) = blacklist.get(guild_id)
        && blacklisted.contains(&reaction.channel_id)
    {
        return;
    }

    snipes
        .entry(reaction.channel_id)
        .or_default()
        .push(reaction.clone());
}
