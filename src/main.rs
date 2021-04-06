mod mongoclient;
use mongoclient::MongoClient;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let my_mongo_client = MongoClient::from_environment().await;
    // let collection = MongoClient::collection_wrap(of: "something");
    let coll = MongoClient::proxy_for("db", "collection").await;
    coll.simple_greet("Mongo proxy almost there !").await;
    
    // Interface outline:
    // coll.find_matching(filter) {}
    // coll.update(filter, new_document) {}
    // coll.delete(filter, new_document) {}
    // coll.get_all() {}

    Ok(())
}