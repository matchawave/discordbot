use std::sync::Arc;

use serenity::all::{
    CacheHttp, ChannelId, ChannelType, Context, CreateChannel, Guild, GuildChannel, Member, UserId,
    VoiceState,
};
use utils::{BotStringParser, info};

use crate::{ElapsedTime, UserVoiceConfigRepo, VoiceConfig, VoiceHub, VoiceHubRepo};

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

    if let Some(old) = old
        && let (Some(old_channel_id), _) = (old.channel_id, new.channel_id)
    {
        let Some(channel) = guild.channels.get(&old_channel_id) else {
            return;
        };
        if channel.kind != ChannelType::Voice {
            return;
        }
        info!(
            "{} left voice channel {} in {}({})",
            member.user.name, old_channel_id, guild.name, guild_id
        );
        let master = VoiceMaster::new(ctx, member, &guild, channel);
        match master.handle_channel_deletion().await {
            Ok(_) => {
                info!(
                    "{} deleted voice channel {} for guild: {}({}) ({}ms)",
                    member.user.name,
                    old_channel_id,
                    guild.name,
                    guild_id,
                    timer.elapsed_ms()
                );
            }
            Err(err) => {
                info!(
                    "Failed to delete voice channel for {} in guild {}({}): {} ({}ms)",
                    member.user.name,
                    guild.name,
                    guild_id,
                    err,
                    timer.elapsed_ms()
                );
            }
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
                "{} created voice channel {} for guild: {}({}) ({}ms)",
                member.user.name,
                new_channel.id,
                guild.name,
                guild_id,
                timer.elapsed_ms()
            );
        }
        Err(err) => {
            // Failed to create a new channel
            info!(
                "Failed to create voice channel for {} in guild {}({}): {} ({}ms)",
                member.user.name,
                guild.name,
                guild_id,
                err,
                timer.elapsed_ms()
            );
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

    async fn get_repo(&self) -> Arc<VoiceHubRepo> {
        let data = self.ctx.data.clone();
        let data_read = data.read().await;
        data_read
            .get::<VoiceHub>()
            .cloned()
            .expect("Expected VoiceHub in TypeMap")
    }

    async fn user_config(&self) -> Option<VoiceConfig> {
        let data = self.ctx.data.clone();
        let data_read = data.read().await;
        let user_voice_configs = data_read
            .get::<UserVoiceConfigRepo>()
            .expect("Expected UserVoiceConfig in TypeMap");
        user_voice_configs
            .get(&self.guild.id, &self.member.user.id)
            .map(|v| v.clone())
    }

    pub async fn handle_channel_creation(
        &'a self,
        parser: &'a mut BotStringParser<'a>,
    ) -> Result<(UserId, GuildChannel), String> {
        let repo = self.get_repo().await;

        let Some(voice_master_config) = repo.get(&self.guild.id).map(|v| v.clone()) else {
            return Err("Voice master config not found".into());
        };

        if !voice_master_config.is_master(&self.channel.id) {
            return Err("Not a voice master channel".to_string());
        }
        let server_config = voice_master_config.config().cloned();
        let user_config = self.user_config().await;

        let parent_id = voice_master_config.parent_id();

        // Create the channel configuration in a separate scope to end the mutable borrow
        let channel_config = {
            // Only borrow self mutably for channel creation, then release
            self.create_channel(parent_id, user_config, server_config, parser)
        };
        // After channel_config is created, we can safely clone guild

        match self
            .guild
            .create_channel(self.ctx.http(), channel_config)
            .await
        {
            Ok(new_channel) => {
                match self.move_member(&new_channel).await {
                    Err(why) => {
                        if let Err(why) = new_channel.delete(self.ctx.http()).await {
                            return Err(format!(
                                "Failed to delete channel after move failure: {:?}",
                                why
                            ));
                        } else {
                            return Err(format!("Failed to move member: {:?}", why));
                        }
                    }
                    Ok((id, channel)) => {
                        if let Some(mut config) = repo.get_mut(&self.guild.id) {
                            config.add_active_channel(channel, id);
                        }
                    }
                }

                Ok((self.member.user.id, new_channel))
            }
            Err(why) => Err(format!("Failed to create channel: {:?}", why)),
        }
    }

    pub async fn handle_channel_deletion(&self) -> Result<(), String> {
        let repo = self.get_repo().await;

        let Some(voice_master_config) = repo.get(&self.guild.id).map(|v| v.clone()) else {
            return Err("Voice master config not found".into());
        };

        if voice_master_config.is_master(&self.channel.id) {
            return Err("This is the voice master channel".to_string());
        }

        if !voice_master_config.is_active(&self.channel.id) {
            return Err("Channel is not managed by voice master".to_string());
        }

        let channel = self.channel;
        if channel.kind != ChannelType::Voice {
            return Err("Channel is not a voice channel".to_string());
        }

        let Ok(members) = channel.members(&self.ctx.cache) else {
            return Err("Failed to get channel members".into());
        };

        if !members.is_empty() {
            return Err("Channel is not empty".into());
        }

        if let Err(why) = self.channel.delete(self.ctx.http()).await {
            return Err(format!("Failed to delete channel: {:?}", why));
        }
        if let Some(mut config) = repo.get_mut(&self.guild.id) {
            config.remove_active_channel(&channel.id);
        }
        Ok(())
    }

    fn create_channel(
        &'a self,
        parent_id: Option<&ChannelId>,
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
                    CreateChannel::new(format!("{}'s channel", user.name)).kind(ChannelType::Voice)
                }
            },
        };
        if let Some(parent_id) = parent_id {
            new_channel = new_channel.category(*parent_id);
        } else if let Some(parent) = channel.parent_id {
            new_channel = new_channel.category(parent);
        }

        new_channel
    }

    pub async fn move_member(&self, channel: &GuildChannel) -> Result<(UserId, ChannelId), String> {
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
        Ok((self.member.user.id, channel.id))
    }
}
