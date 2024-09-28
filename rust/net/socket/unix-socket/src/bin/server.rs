use std::{io::Read, os::unix::net::UnixListener};

fn main() -> std::io::Result<()> {
    let socket = UnixListener::bind("test/foo.sock")?;

    for stream in socket.incoming() {
        let mut stream = stream?;
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        println!("{}", buf);
    }
    Ok(())
}
