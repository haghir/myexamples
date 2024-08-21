use std::cell::RefCell;
use std::ops::DerefMut;
use futures_util::StreamExt;
use mysql_async::{Conn, FromRowError, FromValueError, Opts, OptsBuilder, Pool, Result, Row, Value};
use mysql_async::prelude::*;
use sha2::{Sha256, Digest};
use time::PrimitiveDateTime;

#[derive(Clone, Debug)]
pub(crate) enum Gender {
    Male,
    Female,
    Others,
}

impl From<Gender> for Value {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => Value::Int(0),
            Gender::Female => Value::Int(1),
            Gender::Others => Value::Int(2),
        }
    }
}

impl TryFrom<Value> for Gender {
    type Error = FromValueError;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::Int(n) => match n {
                0 => Ok(Gender::Male),
                1 => Ok(Gender::Female),
                2 => Ok(Gender::Others),
                _ => Err(FromValueError(value))
            }
            _ => Err(FromValueError(value))
        }
    }
}

impl FromValue for Gender {
    type Intermediate = Gender;
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    id: u64,
    name: String,
    age: Option<i32>,
    gender: Gender,
    hash: Option<String>,
    available: bool,
    data: Option<Vec<u8>>,
    created_at: Option<PrimitiveDateTime>,
}

impl Person {
    fn new(name: &str, age: Option<i32>, gender: Gender) -> Self {
        Person {
            id: 0,
            name: name.to_string(),
            age,
            gender,
            hash: None,
            available: true,
            data: Some(vec![1, 2, 4, 8, 16]),
            created_at: None,
        }
    }
}

impl FromRow for Person {
    fn from_row_opt(row: Row) -> std::result::Result<Self, FromRowError> {
        Ok(Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2).unwrap(),
            gender: row.get(3).unwrap(),
            hash: row.get(4).unwrap(),
            available: row.get(5).unwrap(),
            data: row.get(6),
            created_at: row.get(7),
        })
    }
}

fn set_hash(person: &mut Person) {
    let mut hasher = Sha256::new();
    let name = person.name.as_bytes();

    hasher.update(name);

    person.hash = Some(hex::encode(hasher.finalize()).to_string());
}

async fn delete(conn: &RefCell<Conn>) -> Result<()> {
    "DELETE FROM people"
        .ignore(conn.borrow_mut().deref_mut())
        .await
}

async fn insert(conn: &RefCell<Conn>) -> Result<()> {
    let mut people = vec! [
        Person::new("Alice", Some(14), Gender::Female),
        Person::new("Bobby", None, Gender::Male),
        Person::new("Chris", Some(37), Gender::Others),
    ];

    for person in &mut people {
        set_hash(person);
    }

    "INSERT INTO people (name, age, gender, hash, data) VALUES (:name, :age, :gender, :hash, :data)"
        .with(people.iter().map(|person| params! {
            "name" => person.name.as_str(),
            "age" => person.age,
            "gender" => &person.gender,
            "hash" => person.hash.as_ref(),
            "data" => person.data.as_ref(),
        }))
        .batch(conn.borrow_mut().deref_mut())
        .await
}

async fn select(conn: &RefCell<Conn>) -> Result<()> {
    let query = r#"SELECT
        id
    ,   name
    ,   age
    ,   gender
    ,   hash
    ,   available
    ,   data
    ,   created_at
    FROM
        people
    WHERE
        name LIKE :name
    ;"#.with(params! {
        "name" => "%i%"
    });

    let mut ref_conn = conn.borrow_mut();
    let deref_conn = ref_conn.deref_mut();
    let mut result = query.run(deref_conn).await?;
    if let Some(mut stream) = result.stream().await? {
        while let Some(found) = stream.next().await {
            let found: Person = found?;
            println!("{:?}", found);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opts = OptsBuilder::default()
        .ip_or_hostname("127.0.0.1")
        .tcp_port(3306)
        .user(Some("example"))
        .pass(Some("example"))
        .db_name(Some("example"))
        .into();

    let pool = Pool::new::<Opts>(opts);
    let conn = RefCell::new(pool.get_conn().await?);

    delete(&conn).await?;
    insert(&conn).await?;
    select(&conn).await?;

    drop(conn);
    pool.disconnect().await?;

    Ok(())
}
