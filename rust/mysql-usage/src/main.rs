mod wrapper;
mod person;

use sha2::{Sha256, Digest};
use mysql_async::*;
use mysql_async::prelude::*;
use time::PrimitiveDateTime;

use crate::person::{FindPeople};
use crate::wrapper::SelectQuery;

#[derive(Debug)]
struct Person {
    _id: u64,
    name: String,
    age: Option<i32>,
    hash: Option<String>,
    _available: bool,
    _created_at: Option<PrimitiveDateTime>,
}

impl Person {
    fn new(name: &str, age: Option<i32>) -> Self {
        Person {
            _id: 0,
            name: name.to_string(),
            age,
            hash: None,
            _available: true,
            _created_at: None,
        }
    }
}

impl FromRow for Person {
    fn from_row_opt(mut row: Row) -> core::result::Result<Self, FromRowError> {
        Ok(Person {
            _id: row.take(0).unwrap(),
            name: row.take(1).unwrap(),
            age: row.take(2).unwrap(),
            hash: row.take(3).unwrap(),
            _available: row.take(4).unwrap(),
            _created_at: row.take(5),
        })
    }
}

fn set_hash(person: &mut Person) {
    let mut hasher = Sha256::new();
    let name = person.name.as_bytes();

    hasher.update(name);

    person.hash = Some(hex::encode(hasher.finalize()).to_string());
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut people = vec! [
        Person::new("Alice", Some(14)),
        Person::new("Bobby", None),
        Person::new("Crith", Some(37)),
    ];

    for person in &mut people {
        set_hash(person);
    }

    let opts = OptsBuilder::default()
        .ip_or_hostname("127.0.0.1")
        .tcp_port(3306)
        .user(Some("example"))
        .pass(Some("example"))
        .db_name(Some("example"))
        .into();

    let pool = mysql_async::Pool::new::<Opts>(opts);
    let mut conn = pool.get_conn().await?;

    "DELETE FROM people"
        .ignore(&mut conn)
        .await?;

    "INSERT INTO people (name, age, hash) VALUES (:name, :age, :hash)"
        .with(people.iter().map(|person| params! {
            "name" => person.name.as_str(),
            "age" => person.age,
            "hash" => person.hash.as_ref(),
        }))
        .batch(&mut conn)
        .await?;

    let query = FindPeople {
        name: "%i%".to_string(),
    };
    let mut result = query.select(&mut conn).await?;
    if let Some(mut stream) = result.stream().await? {
        while let Some(found) = stream.next().await {
            let found = found?;
            println!("{:?}", found);
        }
    }

    drop(conn);
    pool.disconnect().await?;

    Ok(())
}
