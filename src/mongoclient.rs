use mongodb::{Client, options::ClientOptions};

#[derive(Clone, Debug)]
pub struct MongoClient {
    pub client: mongodb::Client
}

impl MongoClient {
    pub async fn from_environment() -> Self {
        // build connection options
        let connection_string = parse_environment_vars();
        let mut client_options = ClientOptions::parse(&connection_string).await.unwrap();
        client_options.app_name = Some("RustMongo".to_string());

        let new_mongo_client = Client::with_options(client_options).unwrap(); 

        // initialize new client
        println!("Client created from the environment, connection string is: {}", connection_string);
        MongoClient{ client: new_mongo_client }
    }
    // // pub async fn find_matching(filter) {}
    // // pub async fn update(filter, new_document) {}
    // // pub async fn delete(filter, new_document) {}
    // // pub async fn get_all() {}
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