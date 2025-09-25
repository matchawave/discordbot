use serenity::Client;
use utils::Data;

mod pagination;
mod websocket;

pub async fn initialize_processes(client: &Client) {
    tokio::spawn(pagination::handle_pagination_timeout_loop(
        client.data.clone(),
        client.http.clone(),
    ));
}
