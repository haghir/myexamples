use mongodb::{Client, error::Error, Collection};
use mongodb::options::{ClientOptions, Credential};
use mongodb::results::{InsertManyResult};
use mongodb::bson::{doc};

async fn connect() -> Result<Client, Error> {
    let mut opt = ClientOptions::parse("mongodb://localhost:27017").await?;
    opt.credential = Some(
        Credential::builder()
            .username(String::from("mydbuser"))
            .password(String::from("mydbuser"))
            .build());
    Client::with_options(opt)
}

async fn insert(col: &Collection) -> Result<InsertManyResult, Error> {
    let docs = vec! [
        doc! { "name": "Alice", "age": 20 },
        doc! { "name": "Bobby", "age": 25 },
    ];
    col.insert_many(docs, None).await
}

#[tokio::main]
async fn main() {
    let client = connect().await.unwrap();
    let db = client.database("mydb");
    let col = db.collection("foo");

    insert(&col).await.unwrap();
}
