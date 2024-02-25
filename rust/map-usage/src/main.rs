fn foo<'a>(_: &'a str) -> Result<(), ()> {
    Ok(())
}

fn main() -> Result<(), ()> {
    let x = foo("abc").map(|_| 123)?;
    println!("{:?}", x);
    Ok(())
}
