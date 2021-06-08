mod mongoclient;
use mongoclient::MongoClient;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mongo_wrapper = MongoClient::proxy_for("rust_test", "test_coll").await;
    
    // coll.simple_greet("Mongo proxy almost there !").await;
    mongo_wrapper.find_matching().await;
    
    // Interface outline:
    // coll.find_matching(filter) {}
    // coll.update(filter, new_document) {}
    // coll.delete(filter, new_document) {}
    // coll.get_all() {}

    Ok(())
}