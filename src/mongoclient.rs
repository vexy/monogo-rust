use mongodb::{Client, options::ClientOptions, Collection};

#[derive(Clone, Debug)]
pub struct MongoClient {
    pub collection: Collection
}

impl MongoClient {
    pub async fn proxy_for(database: &str, collection: &str) -> Self {
        // build connection options
        let connection_string = parse_environment_vars();
        let mut client_options = ClientOptions::parse(&connection_string).await.unwrap();
        client_options.app_name = Some("RustMongo".to_string());

        // initialize new client
        println!("Initializing client with connection string: {}", connection_string);
        match Client::with_options(client_options) {
            Ok(new_client) => {
                // create database and collection proxy
                let collection_proxy = new_client.database(database).collection(collection);
                MongoClient{collection: collection_proxy }
            },
            Err(e) => { panic!("Unable to proxy with specified parameters.\nOriginal error: {}", e) }
        }
    }

    pub async fn simple_greet(&self, with: &str) {
        println!("Greetings wtih: {}", with);
    }
}


fn parse_environment_vars() -> String {
    let username = dotenv::var("USERNAME").expect("Username variable not found...").to_owned();
    let pwd = dotenv::var("PASSWORD").expect("Password variable not found...").to_owned();
    let host = dotenv::var("DB_HOST").expect("DB host variable not found...").to_owned();
    let auth_db = dotenv::var("AUTH_DB").expect("Auth DB host variable not found...").to_owned();

    // construct connection string as follows:
    // "mongodb(+srv)://<username>:<pwd>@<db_host>/<authDB>" optionally add at the end "?w=majority"
    let formatted_connection_string = format!("mongodb://{}:{}@{}/{}", username, pwd, host, auth_db);
    return formatted_connection_string;
}