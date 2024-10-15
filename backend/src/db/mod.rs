use mongodb::{Client, Database};

pub async fn init_db() -> Database {
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    client.database("bug_tracking")
}
