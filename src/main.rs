use mongodb::{Client, options::ClientOptions};
use std::io;

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    test();
    Ok(())
}

async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
 }