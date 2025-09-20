use serenity::all::{Context, VoiceState};

pub async fn state_update(ctx: Context, voice_state: VoiceState) {
    let Some(guild_id) = voice_state.guild_id else {
        return;
    };
    let guild = ctx.cache.guild(guild_id).map(|g| g.clone());
    let old_state = guild.and_then(|g| g.voice_states.get(&voice_state.user_id).cloned());
}
