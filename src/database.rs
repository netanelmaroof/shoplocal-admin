use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn start() -> Result<mongodb::Database, &'static str> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let client_options = ClientOptions::parse(&database_url).await.unwrap();

    let client = Client::with_options(client_options).unwrap();

    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
    let db = client.database(&database_name);

    Ok(db)
}