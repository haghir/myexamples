use std::fmt::Display;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MyError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Err(MyError {
        message: "Something went wrong.".to_string(),
    })?
}
