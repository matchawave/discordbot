use main::{Env, create_client};

#[tokio::main]
async fn main() {
    let env = Env::default();
    let mut client = create_client(env).await;

    if let Err(e) = client.start_shards(1).await {
        eprintln!("Error starting client: {}", e);
    }
}
