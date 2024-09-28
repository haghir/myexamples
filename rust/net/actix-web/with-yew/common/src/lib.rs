use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Request {
    pub pin: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Error {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Response {
    pub result: Result<User, Error>,
}

impl Response {
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            result: Ok(User {
                name: String::from(name),
                age,
            }),
        }
    }

    pub fn err(message: &str) -> Self {
        Self {
            result: Err(Error {
                message: String::from(message),
            }),
        }
    }
}
