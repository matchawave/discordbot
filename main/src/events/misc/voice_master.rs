use std::sync::Arc;

use serenity::{
    all::{
        CacheHttp, ChannelId, ChannelType, Context, CreateChannel, Guild, GuildChannel, Member,
        UserId, VoiceState,
    },
    futures::channel,
};
use tokio::sync::RwLock;
use utils::{BotStringParser, info};

use crate::{ElapsedTime, UserVoiceConfigRepo, VoiceConfig, VoiceHub, VoiceMasterConfig};
type Config = Arc<RwLock<VoiceConfig>>;

struct VoiceMaster<'a> {
    member: &'a Member,
    guild: &'a Guild,
    channel: &'a GuildChannel,
    ctx: &'a Context,
}

pub async fn handle_voice_master(ctx: &Context, old: &Option<VoiceState>, new: &VoiceState) {
    let timer = ElapsedTime::new();
    let Some(guild_id) = new.guild_id else { return };

    let member = match &new.member {
        Some(member) => member,
        None => return,
    };

    let guild = match guild_id.to_guild_cached(&ctx.cache) {
        Some(guild) => guild.clone(),
        None => return,
    };

    if let Some(old) = old {
        match (old.channel_id, new.channel_id) {
            (_, Some(_)) => {}
            (Some(old_channel_id), None) => {
                let Some(channel) = guild.channels.get(&old_channel_id) else {
                    return;
                };
                if channel.kind != ChannelType::Voice {
                    return;
                }
                info!(
                    "{} left voice channel {} in {}",
                    member.user.name, old_channel_id, guild.name
                );
                let master = VoiceMaster::new(ctx, member, &guild, channel);
            }
            _ => return,
        }
    }
    let Some(channel_id) = new.channel_id else {
        return;
    };
    let Some(channel) = guild.channels.get(&channel_id) else {
        return;
    };
    let master = VoiceMaster::new(ctx, member, &guild, channel);
    let parser = &mut BotStringParser::new(ctx, &guild, channel, member);
    match master.handle_channel_creation(parser).await {
        Ok((_, new_channel)) => {
            info!(
                "{} created voice channel {} for guild: {} ({}ms)",
                member.user.name,
                new_channel.id,
                guild.name,
                timer.elapsed_ms()
            );
        }
        Err(err) => {
            // Failed to create a new channel
        }
    }
}

impl<'a> VoiceMaster<'a> {
    pub fn new(
        ctx: &'a Context,
        member: &'a Member,
        guild: &'a Guild,
        channel: &'a GuildChannel,
    ) -> Self {
        Self {
            ctx,
            member,
            guild,
            channel,
        }
    }

    async fn voice_master_config(&self) -> Option<Arc<RwLock<VoiceMasterConfig>>> {
        let data = self.ctx.data.clone();
        let data_read = data.read().await;
        let voice_hubs = data_read
            .get::<VoiceHub>()
            .expect("Expected VoiceHub in TypeMap");

        let voice_hub_lock = voice_hubs.read().await;

        voice_hub_lock.get(&self.guild.id).clone()
    }

    async fn user_config(&self) -> Option<Arc<RwLock<VoiceConfig>>> {
        let data = self.ctx.data.clone();
        let data_read = data.read().await;
        let user_voice_configs = data_read
            .get::<UserVoiceConfigRepo>()
            .expect("Expected UserVoiceConfig in TypeMap");

        let user_voice_config = user_voice_configs.read().await;
        user_voice_config
            .get(&self.guild.id, &self.member.user.id)
            .clone()
    }

    pub async fn handle_channel_creation(
        &'a self,
        parser: &'a mut BotStringParser<'a>,
    ) -> Result<(UserId, GuildChannel), String> {
        match self.voice_master_config().await {
            Some(hub) => {
                let voice_master_config = hub.read().await.clone();
                if !voice_master_config.is_master(&self.channel.id) {
                    return Err("Not a voice master channel".to_string());
                }
                let server_config = voice_master_config.config().cloned();
                let user_config = {
                    let user_config = self.user_config().await.clone();
                    if let Some(config) = user_config {
                        Some(config.read().await.clone())
                    } else {
                        None
                    }
                };

                // Create the channel configuration in a separate scope to end the mutable borrow
                let channel_config = {
                    // Only borrow self mutably for channel creation, then release
                    self.create_channel(user_config, server_config, parser)
                };
                // After channel_config is created, we can safely clone guild

                match self
                    .guild
                    .create_channel(self.ctx.http(), channel_config)
                    .await
                {
                    Ok(new_channel) => {
                        self.move_member(&new_channel).await?;
                        Ok((self.member.user.id, new_channel))
                    }
                    Err(why) => Err(format!("Failed to create channel: {:?}", why)),
                }
            }
            None => Err("Voice master config not found".into()),
        }
    }

    fn create_channel(
        &'a self,
        user_config: Option<VoiceConfig>,
        server_config: Option<VoiceConfig>,
        parser: &'a mut BotStringParser<'a>,
    ) -> CreateChannel<'a> {
        let user = &self.member.user;
        let channel = &self.channel;
        let mut new_channel = match server_config {
            Some(config) => {
                let new_channel =
                    CreateChannel::new(format!("{}'s channel", user.name)).kind(ChannelType::Voice);
                config.to_channel(parser, new_channel)
            }
            None => match user_config {
                Some(config) => {
                    let new_channel = CreateChannel::new(format!("{}'s channel", user.name))
                        .kind(ChannelType::Voice);
                    config.to_channel(parser, new_channel)
                }
                None => {
                    let new_channel = CreateChannel::new(format!("{}'s channel", user.name))
                        .kind(ChannelType::Voice);

                    new_channel
                }
            },
        };
        if let Some(parent) = channel.parent_id {
            new_channel = new_channel.category(parent);
        }

        new_channel
    }

    pub async fn store_active_channel(&self, owner: &UserId, channel: &ChannelId) {
        let Some(voice_master_config) = self.voice_master_config().await else {
            return;
        };
        let mut master = voice_master_config.write().await;
        master.add_active_channel(*channel, *owner);
    }

    pub async fn move_member(&self, channel: &GuildChannel) -> Result<(), String> {
        if let Err(why) = self
            .member
            .move_to_voice_channel(self.ctx.http(), channel)
            .await
        {
            return if let Err(why) = channel.delete(self.ctx.http()).await {
                Err(format!(
                    "Failed to delete channel after move failure: {:?}",
                    why
                ))
            } else {
                Err(format!("Failed to move member: {:?}", why))
            };
        }
        let id = self.member.user.id;
        let channel = channel.id;
        self.store_active_channel(&id, &channel).await;
        Ok(())
    }
}
