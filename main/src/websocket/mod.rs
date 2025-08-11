use std::sync::Arc;

use serenity::{
    futures::{
        StreamExt,
        stream::{SplitSink, SplitStream},
    },
    prelude::TypeMapKey,
};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    Connector, MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{ClientRequestBuilder, Message, Utf8Bytes},
};
use utils::{Data, error};

pub type SocketReader<S> = SplitStream<WebSocketStream<S>>;
pub type SocketWriter<S> = SplitSink<WebSocketStream<S>, Message>;

#[derive(Default)]
pub struct TlsConnector(pub Mutex<Option<Connector>>);

#[derive(Default)]
pub struct WebsocketConnection(pub Mutex<Option<SocketWriter<MaybeTlsStream<TcpStream>>>>);

pub struct WebSocketClient {
    data: Data,
}

impl WebSocketClient {
    pub fn new(data: Data) -> Self {
        Self { data }
    }
    pub async fn connect(
        &self,
        req: ClientRequestBuilder,
    ) -> Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>> {
        let Ok((ws_stream, _)) = connect_async(req).await else {
            error!("Failed to connect to websocket");
            return None;
        };

        let (write, read) = ws_stream.split();
        self.data
            .write()
            .await
            .insert::<WebSocket>(WebsocketConnection(Mutex::new(Some(write))));
        Some(read)
    }
    pub async fn handle_ws_message(&self, data: Utf8Bytes) {
        println!("Received websocket message: {}", data);
    }
    pub async fn handle_disconnect(&self) {
        self.data.write().await.remove::<WebSocket>();
    }
}

pub struct WebSocket;
impl TypeMapKey for WebSocket {
    type Value = WebsocketConnection;
}
