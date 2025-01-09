use mongodb::{Client, options::ClientOptions, Collection};
use std::env;
use dotenv::dotenv;
use crate::DataFrame;

pub async fn connect_to_mongodb() -> mongodb::error::Result<Client> {
    dotenv().ok(); // Load enviroment vairables

    let uri = env::var("MONGODB_URI")
        .expect("MONGODB_URI must be set in .env file");

    let client_options = ClientOptions::parse(uri).await?;
    Client::with_options(client_options)
}

pub async fn save_to_mongodb(client: &Client, frame: &DataFrame)
    -> mongodb::error::Result<()> 
{
    let db = client.database("slime_watcher");
    let collection: Collection<DataFrame> = db.collection("input_data");

    collection.insert_one(frame, None).await?; // Insert data
    println!("Data saved to MongoDB");

    Ok(())
}