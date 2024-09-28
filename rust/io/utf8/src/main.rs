use std::fs::File;
use std::io::{Error, ErrorKind, Read};

struct CharStream<T: Read> {
    stream: T,
    buf: [u8; 32],
    idx: usize,
    len: usize,
}

impl<T: Read> CharStream<T> {
    fn new(stream: T) -> Self {
        Self {
            stream,
            buf: [0; 32],
            idx: 0,
            len: 0,
        }
    }

    fn fetch(&mut self) -> std::io::Result<Option<char>> {
        let mut state = 0;
        let mut value = 0;

        loop {
            let b = if let Some(ob) = self.read_byte()? {
                ob
            } else if state == 0 {
                return Ok(None);
            } else {
                return Err(Error::from(ErrorKind::InvalidInput));
            };

            match state {
                0 => {
                    if (b & 0x80) == 0 {
                        value = b as u32;
                        state = 0;
                    } else if (b & 0xe0) == 0xc0 {
                        value = ((b as u32) & 0x1f) << 6;
                        state = 1;
                    } else if (b & 0xf0) == 0xe0 {
                        value = ((b as u32) & 0xf) << 12;
                        state = 2;
                    } else if (b & 0xf8) == 0xf0 {
                        value = ((b as u32) & 0x7) << 18;
                        state = 3;
                    } else {
                        return Err(Error::from(ErrorKind::InvalidInput));
                    }
                },
                1 | 2 | 3 => {
                    if (b & 0xc0) == 0x80 {
                        value |= ((b as u32) & 0x3f) << ((state - 1) * 6);
                        state -= 1;
                    } else {
                        return Err(Error::from(ErrorKind::InvalidInput));
                    }
                },
                _ => {
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
            }

            if state == 0 {
                return Ok(char::from_u32(value));
            }
        }
    }

    fn read_byte(&mut self) -> std::io::Result<Option<u8>> {
        if self.idx >= self.len {
            let size = self.stream.read(&mut self.buf)?;
            self.idx = 0;
            self.len = size;
        }

        if self.len == 0 {
            return Ok(None);
        }

        let b = self.buf[self.idx];
        self.idx += 1;

        Ok(Some(b))
    }
}

fn main() {
    let file = File::open("foo.txt").unwrap();
    let mut stream = CharStream::new(file);

    loop {
        if let Ok(oc) = stream.fetch() {
            if let Some(c) = oc {
                print!("{}", c);
            } else {
                break;
            }
        } else {
            println!("Something went wrong.");
            break;
        }
    }
    println!();
}
