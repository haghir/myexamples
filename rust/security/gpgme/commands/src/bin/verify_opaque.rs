use gpgme::Context;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = Context::from_protocol(gpgme::Protocol::OpenPgp)?;
    ctx.set_engine_home_dir("test/.gnupg")?;

    let mut cipher = File::open("test/signed.gpg")?;
    let mut plain = Vec::new();

    let result = ctx.verify_opaque(&mut cipher, &mut plain)?;
    let plain = String::from_utf8(plain)?;

    println!("{:?}", result);
    println!("{:?}", plain);

    Ok(())
}
