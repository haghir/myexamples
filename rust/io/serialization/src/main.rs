use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    value: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bar {
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Baz {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum DataType {
    FooType(Foo),
    BarType(Bar),
    BazType(Baz),
}

fn main() -> Result<()> {
    let v1 = vec![
        DataType::FooType(Foo { value: 123 }),
        DataType::BarType(Bar { value: "bar".to_string() }),
        DataType::BazType(Baz { id: 789, name: "baz".to_string() }),
    ];
    let json = serde_json::to_string(&v1)?;
    println!("{}", json);

    let v2: Vec<DataType> = serde_json::from_str(json.as_str())?;
    println!("{:?}", v2);

    Ok(())
}
