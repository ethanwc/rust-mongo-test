use mongodb::{Client, options::ClientOptions};
use std::io;
use futures::executor::block_on;
use mongodb::bson::doc;

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    let future = test();
    block_on(future);
    Ok(())
}

async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut client_options = ClientOptions::parse("mongodb+srv://ourrecipes-temp:Vpzk0A4RUxHRsWYB@cluster0.1bw4q.mongodb.net/<dbname>?retryWrites=true&w=majority").await?;
    let client = Client::with_options(client_options)?;

    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
 }