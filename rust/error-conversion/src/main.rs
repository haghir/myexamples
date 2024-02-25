#[derive(Debug)]
struct MyError {
    message: String,
}

#[derive(Debug)]
struct OurError {
    message: String,
}

impl From<OurError> for MyError {
    fn from(error: OurError) -> Self {
        let m = format!("{} (from OurError)", error.message);
        Self { message: String::from(m) }
    }
}

impl From<MyError> for OurError {
    fn from(error: MyError) -> Self {
        let m = format!("{} (from MyError)", error.message);
        Self { message: String::from(m) }
    }
}

fn baz() -> Result<(), MyError> {
    Err(MyError { message: String::from("Something happened") })
}

fn bar() -> Result<(), OurError> {
    baz()?;
    Ok(())
}

fn foo() -> Result<(), MyError> {
    bar()?;
    Ok(())
}

fn main() -> Result<(), OurError> {
    foo()?;
    Ok(())
}
