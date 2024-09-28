use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:8080")?;

    if let Err(e) = client.write_fmt(format_args!("Hello, socket!")) {
        panic!("Failed to send a message: {}", e);
    }

    if let Err(e) = client.shutdown(std::net::Shutdown::Write) {
        panic!("Failed to close the writing stream: {}", e);
    }

    let mut buf = String::new();
    if let Err(e) = client.read_to_string(&mut buf) {
        panic!("Failed to receive a message: {}", e);
    }

    println!("I have received a message: {}", buf);

    Ok(())
}
