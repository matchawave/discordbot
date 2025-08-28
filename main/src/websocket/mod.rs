use std::sync::Arc;

use serenity::{
    all::CurrentUser,
    futures::{
        SinkExt, StreamExt,
        stream::{SplitSink, SplitStream},
    },
    prelude::TypeMapKey,
};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    Connector, MaybeTlsStream, WebSocketStream as WsStream, connect_async,
    tungstenite::{ClientRequestBuilder, Message, Utf8Bytes, http::Uri},
};
use utils::{Data, error, info};

use crate::Env;

type WebSocketStream = WsStream<MaybeTlsStream<TcpStream>>;
pub type SocketReader = SplitStream<WebSocketStream>;
pub type SocketWriter = SplitSink<WebSocketStream, Message>;

pub struct WebSocketWriter(Arc<Mutex<SocketWriter>>);
impl WebSocketWriter {
    pub fn new(writer: SocketWriter) -> Self {
        Self(Arc::new(Mutex::new(writer)))
    }

    pub async fn send(&self, msg: String) {
        let mut writer = self.0.lock().await;
        let msg = Message::Text(Utf8Bytes::from(msg));
        let _ = writer.send(msg).await;
    }
}

pub struct WebSocketInstance {
    req: ClientRequestBuilder,
    data: Data,
}
const PROTOCOL: &str = "ws";
impl WebSocketInstance {
    pub fn new(user: &CurrentUser, env: &Env, data: &Data) -> Self {
        let ws_url = format!("{}://{}/api/gateway/{}", PROTOCOL, env.api_url(), user.id);
        info!("Connecting to {}", "Gateway".yellow());
        let uri = Uri::try_from(ws_url.as_str()).expect("Invalid websocket URI");
        let req = ClientRequestBuilder::new(uri)
            .with_header("client", format!("DiscordBot {}", env.token()));

        Self {
            req,
            data: data.clone(),
        }
    }
    pub async fn connect(self) {
        tokio::spawn(async move {
            loop {
                if let Some(mut reader) = self._connect().await {
                    info!("Connected to {}", "Gateway".yellow());
                    while let Some(Ok(message)) = reader.next().await {
                        match message {
                            Message::Text(text) => self.handle_ws_message(text).await,
                            _ => break,
                        }
                    }

                    error!("Websocket connection closed or error occurred");
                    self.handle_disconnect().await;
                }
                info!("Reconnecting to {}", "Gateway".yellow());
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            }
        });
    }
    async fn _connect(&self) -> Option<SocketReader> {
        let Ok((ws_stream, _)) = connect_async(self.req.clone()).await else {
            error!("Failed to connect to {}", "Gateway".yellow());
            return None;
        };

        let (write, read) = ws_stream.split();
        let writer = WebSocketWriter::new(write);
        self.data.write().await.insert::<WebSocket>(writer);
        Some(read)
    }
    async fn handle_ws_message(&self, data: Utf8Bytes) {
        println!("Rcv ws msg: {}", data);
    }
    async fn handle_disconnect(&self) {
        self.data.write().await.remove::<WebSocket>();
    }
}

pub struct WebSocket;
impl TypeMapKey for WebSocket {
    type Value = WebSocketWriter;
}
