use gpgme::Context;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = Context::from_protocol(gpgme::Protocol::OpenPgp)?;
    ctx.set_engine_home_dir("test/.gnupg")?;

    let sign = File::open("test/detached-sign.gpg")?;
    let orig = File::open("test/original.txt")?;

    let result = ctx.verify_detached(&sign, &orig)?;

    println!("{:?}", result);

    Ok(())
}
