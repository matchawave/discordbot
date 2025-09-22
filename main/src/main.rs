use main::{Env, create_client};

#[tokio::main]
async fn main() {
    let env = Env::default();

    let shards = 1; // Change this to the desired number of shards
    let mut client = create_client(env, shards).await;

    if let Err(e) = client.start_shards(shards as u32).await {
        eprintln!("Error starting client: {}", e);
    }
}
