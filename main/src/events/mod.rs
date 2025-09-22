use std::pin::Pin;

use serenity::{
    all::{Context, Event, RawEventHandler},
    async_trait,
};
use tokio::{
    sync::mpsc::{self, Sender},
    task,
};

mod automod;
mod category;
mod channel;
mod guild;
mod interaction;
mod message;
mod reaction;
mod schedule;
mod stage;
mod state;
mod thread;
mod voice;

pub struct Handler {
    senders: Vec<Sender<(Context, Event)>>,
}

impl Handler {
    pub fn new(shard_count: usize) -> Self {
        let mut senders = Vec::with_capacity(shard_count);
        for _ in 0..shard_count {
            let (send, recv) = mpsc::channel(10_000);
            tokio::spawn(worker(recv));
            senders.push(send);
        }
        Self { senders }
    }
}

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, ctx: Context, ev: Event) {
        let shard_id = ctx.shard_id.get() as usize;

        if let Some(sender) = &self.senders.get(shard_id) {
            sender.send((ctx, ev)).await.unwrap_or_else(|e| {
                eprintln!("Error sending event to shard {}: {}", shard_id, e);
            });
            return;
        }
    }
}

#[rustfmt::skip]
async fn worker(mut receiver: mpsc::Receiver<(Context, Event)>) {
    while let Some((ctx, ev)) = receiver.recv().await {
        task::spawn({
            let ctx = ctx.clone();
            async move {
                match ev {
                    Event::MessageCreate(ev) => message::create(ctx, ev.message).await,
                    Event::MessageUpdate(ev) => message::update(ctx, ev).await,
                    Event::MessageDelete(ev) => message::delete(ctx, ev.channel_id, ev.message_id, ev.guild_id).await,
                    Event::MessageDeleteBulk(ev) => message::bulk_delete(ctx, ev.channel_id, ev.guild_id, ev.ids).await,
                    Event::InteractionCreate(ev) => interaction::create(ctx, ev.interaction).await,
                    Event::Ready(ev) => state::ready(ctx, ev.ready).await,
                    Event::GuildCreate(ev) => guild::create(ctx, ev.guild).await,
                    Event::GuildDelete(ev) => guild::delete(ctx, ev.guild).await,
                    Event::VoiceStateUpdate(ev) => voice::state_update(ctx, ev.voice_state).await,
                    _ => empty_handler().await,
                }
            }
        });
    }
}

async fn empty_handler() {
    // do nothing
}
