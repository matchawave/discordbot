use serenity::all::{ChannelType, Context, CreateChannel, GuildChannel, User, VoiceState};

use crate::{ActiveVoiceChannel, UserVoiceConfig, UserVoiceConfigRepo, VoiceHub};

pub async fn handle_voice_master(ctx: &Context, old: &Option<VoiceState>, new: &VoiceState) {
    let data = ctx.data.clone();

    let Some(guild_id) = new.guild_id else { return };

    let member = match &new.member {
        Some(member) => member,
        None => return,
    };

    let (voice_hub, user_voice_configs) = {
        let data_read = data.read().await;
        let voice_hubs = data_read
            .get::<VoiceHub>()
            .expect("Expected VoiceHub in TypeMap");

        let user_voice_configs = data_read
            .get::<UserVoiceConfigRepo>()
            .expect("Expected UserVoiceConfig in TypeMap");

        let voice_hub_lock = voice_hubs.read().await;
        let user_voice_configs = user_voice_configs.read().await;
        (
            voice_hub_lock.get(&guild_id).clone(),
            user_voice_configs.get(&guild_id, &new.user_id).clone(),
        )
    };

    if let Some(hub) = voice_hub {
        // let hub = hub.clone();
        let config = {
            let hub = hub.clone();
            let hub_read = hub.read().await;
            hub_read.clone()
        };
        match old {
            Some(old) => match (old.channel_id, new.channel_id) {
                (_, Some(new_channel_id)) => {}
                (Some(old_channel_id), None) => {}
                _ => {}
            },
            None => {
                let Some(new_channel_id) = new.channel_id else {
                    return;
                };
                if !config.is_master(&new_channel_id) {
                    return;
                }
                // Handle new channel creation
            }
        }
    }
}

async fn handle_channel_creation(ctx: &Context) -> Result<ActiveVoiceChannel, String> {
    Err("Not implemented".to_string())
}
async fn handle_channel_deletion() -> Result<Option<ActiveVoiceChannel>, String> {
    Err("Not implemented".to_string())
}

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
