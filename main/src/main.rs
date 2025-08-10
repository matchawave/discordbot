use main::{Env, create_client, register_commands};

#[tokio::main]
async fn main() {
    let env = Env::default();
    let (commands, commands_map) = register_commands();
    let mut client = create_client(env, commands_map).await;

    if let Err(e) = client.start_shards(1).await {
        eprintln!("Error starting client: {}", e);
    }
}
