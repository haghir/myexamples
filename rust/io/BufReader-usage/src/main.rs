use std::io::{BufRead, BufReader};

fn read_line<T: BufRead>(reader: &mut T) {
    let mut line: String = Default::default();
    println!("{:?}", reader.read_line(&mut line));
    println!("{:?}", line);
}

fn main() {
    let text = "abc\n\ndef\nghi".as_bytes();
    let mut reader = BufReader::new(text);
    read_line(&mut reader);
    read_line(&mut reader);
    read_line(&mut reader);
    read_line(&mut reader);
    read_line(&mut reader);
    read_line(&mut reader);
}
