use fs2::FileExt;
use std::{fs::File, io::Read, thread::sleep, time::Duration};

fn main() -> std::io::Result<()> {
    let dur = Duration::from_millis(3000);
    let mut file = File::open("test/foo.txt")?;
    file.lock_shared()?;
    println!("locking...");

    sleep(dur);

    let mut text = String::new();
    if let Err(e) = file.read_to_string(&mut text) {
        return Err(e);
    }
    println!("{}", text);

    Ok(())
}
