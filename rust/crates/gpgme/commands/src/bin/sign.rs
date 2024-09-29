use gpgme::{Context, SignMode};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = Context::from_protocol(gpgme::Protocol::OpenPgp)?;
    ctx.set_engine_home_dir("test/.gnupg")?;
    ctx.set_armor(true);

    // sign original.txt
    let plain = File::open("test/original.txt")?;
    let mut signature = Vec::new();
    let result = ctx.sign(SignMode::Normal, &plain, &mut signature)?;
    let signature = String::from_utf8(signature)?;

    println!("{:?}", result);
    println!("{:?}", signature);

    // verify the signagure
    let mut plain = Vec::new();
    let result = ctx.verify_opaque(signature.as_bytes(), &mut plain)?;
    let plain = String::from_utf8(plain)?;

    println!("{:?}", result);
    println!("{:?}", plain);

    Ok(())
}
