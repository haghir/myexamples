use std::{io::Write, os::unix::net::UnixStream};

fn main() -> std::io::Result<()> {
    let mut socket = UnixStream::connect("test/foo.sock")?;
    socket.write_fmt(format_args!("Hello, unix socket!"))
}
