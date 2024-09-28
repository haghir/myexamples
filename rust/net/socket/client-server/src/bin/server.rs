use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let mut message = String::new();
    if let Err(e) = stream.read_to_string(&mut message) {
        println!("Failed to receive: {}", e);
        return;
    }
    println!("I have received a message: {}", message);

    if let Err(e) = stream.write_fmt(format_args!("I have received the message: {}", message)) {
        println!("Failed to return: {}", e);
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("Something went wrong: {}", e),
        }
    }
    Ok(())
}
