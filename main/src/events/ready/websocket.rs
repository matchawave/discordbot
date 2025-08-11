use serenity::futures::StreamExt;
use tokio_tungstenite::tungstenite::{ClientRequestBuilder, Message, http::Uri};
use utils::{Data, error, info};

use crate::{Environment, websocket::WebSocketClient};

const PROTOCOL: &str = "ws";
pub async fn run(data: Data, client_id: String) {
    let websocket = WebSocketClient::new(data.clone());
    let environment = {
        let data = data.read().await;
        data.get::<Environment>().cloned().unwrap_or_default()
    };

    let ws_url = format!(
        "{}://{}/api/gateway/{}",
        PROTOCOL,
        environment.api_url(),
        client_id
    );

    let uri = Uri::try_from(ws_url.as_str()).expect("Invalid websocket URI");
    let req = ClientRequestBuilder::new(uri)
        .with_header("client", format!("DiscordBot {}", environment.token()));

    info!("Connecting to websocket at {}", ws_url);
    loop {
        if let Some(mut reader) = websocket.connect(req.clone()).await {
            info!("Connected to websocket");
            while let Some(Ok(message)) = reader.next().await {
                match message {
                    Message::Text(text) => websocket.handle_ws_message(text).await,
                    _ => break,
                }
            }
            error!("Websocket connection closed or error occurred");
            websocket.handle_disconnect().await;
        }
    }
}
