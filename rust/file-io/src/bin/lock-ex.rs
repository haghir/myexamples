use fs2::FileExt;
use std::{fs::File, io::Write, thread::sleep, time::Duration};

fn main() -> std::io::Result<()> {
    let dur = Duration::from_millis(10000);
    let mut file = File::create("test/foo.txt")?;
    file.lock_exclusive()?;
    println!("locking...");

    file.write("foo".as_bytes())?;

    sleep(dur);

    file.write("foo".as_bytes())?;

    Ok(())
}
