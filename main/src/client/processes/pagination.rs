use std::sync::Arc;

use serenity::all::{EditMessage, Http};
use utils::{Data, info};

use crate::{ElapsedTime, Paginations};

pub async fn handle_pagination_timeout_loop(data: Data, http: Arc<Http>) {
    info!("Started pagination timeout loop.");
    loop {
        let paginations = {
            let data = data.read().await;
            data.get::<Paginations>()
                .expect("Expected Paginations in TypeMap.")
                .clone()
        };
        for (key, page_lock) in paginations.map().await.iter() {
            let pagination = page_lock.read().await.clone();

            if pagination.is_expired()
                && let Some((channel_id, message_id)) = pagination.id()
            {
                let timer = ElapsedTime::new();
                let embed = pagination.current().0;
                match http
                    .edit_message(
                        channel_id,
                        message_id,
                        &EditMessage::new().embed(embed).components(vec![]),
                        vec![],
                    )
                    .await
                {
                    Ok(_) => {
                        paginations.remove(key).await;
                        info!(
                            "Removed expired pagination with key {} ({} ms)",
                            key,
                            timer.elapsed_ms()
                        );
                    }
                    Err(e) => info!("Failed to remove pagination components: {}", e),
                };
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}
