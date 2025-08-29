use serenity::all::{ChannelType, Context, CreateChannel, GuildChannel, User, VoiceState};

use crate::{ActiveVoiceChannel, VoiceHub};

pub async fn handle_voice_master(ctx: &Context, old: &Option<VoiceState>, new: &VoiceState) {
    let data = ctx.data.clone();

    let (guild_id, channel_id) = match (new.guild_id, new.channel_id, old.as_ref()) {
        (Some(guild_id), Some(channel_id), _) => (guild_id, channel_id),
        (Some(guild_id), None, Some(old)) => match old.channel_id {
            Some(old_channel_id) => (guild_id, old_channel_id),
            None => return,
        },
        _ => return,
    };

    let member = match &new.member {
        Some(member) => member,
        None => return,
    };

    let voice_hub = {
        let data_read = data.read().await;
        let voice_hubs = data_read
            .get::<VoiceHub>()
            .expect("Expected VoiceHub in TypeMap");

        let voice_hub_lock = voice_hubs.read().await;
        voice_hub_lock.get(&guild_id).clone()
    };

    if let Some(hub) = voice_hub {
        // let hub = hub.clone();
        let config = {
            let hub = hub.clone();
            let hub_read = hub.read().await;
            hub_read.clone()
        };
        match old {
            Some(old) => {
                if old.guild_id != new.guild_id || old.channel_id == new.channel_id {
                    return;
                }
                match (old.channel_id, new.channel_id) {
                    (_, Some(_)) => {}
                    (Some(old_channel_id), None) => {}
                    _ => {}
                }
            }
            None => if config.is_master(&channel_id) {},
        }
    }
}

async fn handle_channel_creation() -> Result<ActiveVoiceChannel, String> {}
async fn handle_channel_deletion() -> Result<Option<ActiveVoiceChannel>, String> {}

fn create_channel<'a>(user: &'a User, channel: &'a GuildChannel) -> CreateChannel<'a> {
    let channel_name = format!("{}-room", user.name);
    let mut new_channel = CreateChannel::new(channel_name)
        .position(channel.position + 1)
        .kind(ChannelType::Voice);
    if let Some(category) = channel.parent_id {
        new_channel = new_channel.category(category);
    }
    if let Some(bitrate) = channel.bitrate {
        new_channel = new_channel.bitrate(bitrate);
    }
    new_channel
}
