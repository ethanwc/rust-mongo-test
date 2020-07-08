use futures::executor::block_on;
use std::io;

use mongodb::{
    bson::{doc, Bson},
    sync::Client,
};

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    let future = test2();
    block_on(future);
    Ok(())
}

async fn test2() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_uri_str("mongodb+srv://ourrecipes-temp:Vpzk0A4RUxHRsWYB@cluster0.1bw4q.mongodb.net")?;
    let database = client.database("ourrecipes");
    let collection = database.collection("books");
    
    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];
    
    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None)?;
    
    let cursor = collection.find(doc! { "author": "George Orwell" }, None)?;
    for result in cursor {
        match result {
            Ok(document) => {
                if let Some(title) = document.get("title").and_then(Bson::as_str) {
                    println!("title: {}", title);
                } else {
                    println!("no title found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}