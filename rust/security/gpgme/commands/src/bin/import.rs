use gpgme::Context;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = Context::from_protocol(gpgme::Protocol::OpenPgp)?;
    ctx.set_engine_home_dir("test/.gnupg")?;

    let pubkeys = File::open("test/pubkeys.gpg")?;
    let result = ctx.import(&pubkeys)?;

    println!("{:?}", result);

    for import in result.imports() {
        if let Ok(fpr) = import.fingerprint() {
            println!("fingerprint: {}", fpr);
        }
    }

    Ok(())
}
