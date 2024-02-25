use sha2::{Sha256, Digest};
use mysql_async::{prelude::*, Result, Opts, OptsBuilder};
use log::info;
use time::PrimitiveDateTime;

#[derive(Debug)]
struct Person {
    id: u64,
    name: String,
    age: Option<i32>,
    hash: Option<String>,
    available: bool,
    created_at: Option<PrimitiveDateTime>,
}

impl Person {
    fn new(name: &str, age: Option<i32>) -> Self {
        Person {
            id: 0,
            name: name.to_string(),
            age,
            hash: None,
            available: true,
            created_at: None,
        }
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

    let selected = "SELECT id, name, age, hash, available, created_at FROM people"
        .with(())
        .map(&mut conn, |(id, name, age, hash, available, created_at)| {
            Person { id, name, age, hash, available, created_at }
        })
        .await?;

    drop(conn);
    pool.disconnect().await?;

    for person in selected {
        info!("{:?}", person);
    }

    Ok(())
}
