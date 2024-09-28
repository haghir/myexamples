use serde::{Serialize, Deserialize};
use toml::{de::from_str, ser::to_string};

#[derive(Debug, Serialize, Deserialize)]
pub struct Foo {
    pub bar: Bar,
    pub baz: Vec<Baz>,
    pub footext: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bar {
    pub barbool: bool,
    pub barnum: i32,
    pub bartext: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Baz {
    pub bazbool: bool,
    pub baznum: i32,
    pub baztext: String,
}

fn main() {
    let foo = Foo {
        bar: Bar {
            barbool: false,
            barnum: 123,
            bartext: "bar".to_string(),
        },
        baz: vec! [
            Baz {
                bazbool: true,
                baznum: 456,
                baztext: "baz 1".to_string(),
            },
            Baz {
                bazbool: false,
                baznum: 789,
                baztext: "baz 2".to_string(),
            }
        ],
        footext: "foo".to_string(),
    };

    let foostr = to_string(&foo).unwrap();
    println!("{}", foostr);

    let defoo: Foo = from_str(foostr.as_str()).unwrap();
    println!("{:?}", defoo);
}
