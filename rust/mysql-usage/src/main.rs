use futures_util::StreamExt;
use mysql_async::{FromRowError, FromValueError, Opts, OptsBuilder, Pool, Result, Row, Value};
use mysql_async::prelude::*;
use sha2::{Sha256, Digest};
use time::PrimitiveDateTime;

#[derive(Clone, Debug)]
pub(crate) enum Gender {
    Male,
    Female,
    Others,
}

impl Into<Value> for Gender {
    fn into(self) -> Value {
        Value::from(match self {
            Self::Male => Value::Int(0),
            Self::Female => Value::Int(1),
            Self::Others => Value::Int(2),
        })
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

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut people = vec! [
        Person::new("Alice", Some(14), Gender::Female),
        Person::new("Bobby", None, Gender::Male),
        Person::new("Chris", Some(37), Gender::Others),
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

    let pool = Pool::new::<Opts>(opts);
    let mut conn = pool.get_conn().await?;

    "DELETE FROM people"
        .ignore(&mut conn)
        .await?;

    "INSERT INTO people (name, age, gender, hash, data) VALUES (:name, :age, :gender, :hash, :data)"
        .with(people.iter().map(|person| params! {
            "name" => person.name.as_str(),
            "age" => person.age,
            "gender" => &person.gender,
            "hash" => person.hash.as_ref(),
            "data" => person.data.as_ref(),
        }))
        .batch(&mut conn)
        .await?;

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

    let mut result = query.run(&mut conn).await?;
    if let Some(mut stream) = result.stream().await? {
        while let Some(found) = stream.next().await {
            let found: Person = found?;
            println!("{:?}", found);
        }
    }

    drop(conn);
    pool.disconnect().await?;

    Ok(())
}
