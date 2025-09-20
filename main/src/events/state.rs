use serenity::all::{Context, Ready};

use crate::ready;

pub async fn ready(ctx: Context, ready: Ready) {
    ready::handle(ctx, ready).await;
}
