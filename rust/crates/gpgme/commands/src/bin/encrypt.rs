use gpgme::{Context, Key, SignMode};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = Context::from_protocol(gpgme::Protocol::OpenPgp)?;
    ctx.set_engine_home_dir("test/.gnupg")?;
    ctx.set_armor(true);

    // encrypt original.txt
    let plain = File::open("test/original.txt")?;
    let keys: Vec<Key> = ctx
        .find_keys(["foo@example.com"])?
        .filter_map(|x| x.ok())
        .filter(|x| x.can_encrypt())
        .collect();
    let mut cipher = Vec::new();
    let result = ctx.encrypt(&keys, &plain, &mut cipher)?;
    let cipher = String::from_utf8(cipher)?;

    println!("{:?}", result);
    println!("{:?}", cipher);

    // decrypt the cipher
    let mut plain = Vec::new();
    let result = ctx.decrypt(cipher.as_bytes(), &mut plain)?;
    let plain = String::from_utf8(plain)?;

    println!("{:?}", result);
    println!("{:?}", plain);

    Ok(())
}
