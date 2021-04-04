mod mongoclient;

use mongoclient::{MongoClient};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    MongoClient::from_environment().await;
    Ok(())
}