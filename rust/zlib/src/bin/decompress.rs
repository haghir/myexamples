use std::env;
use std::io::prelude::*;
use std::fs::File;
use flate2::read::ZlibDecoder;

fn read(file: &mut impl Read, buf: &mut [u8], size: &mut usize) -> std::io::Result<bool> {
    *size = file.read(buf)?;
    Ok(*size > 0)
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Illegal arguments.");
    }

    let src = File::open(args[1].as_str())?;
    let mut dst = File::create(args[2].as_str())?;
    let mut d = ZlibDecoder::new(src);

    let mut buf: [u8; 1024] = [0; 1024];
    let mut size = 0;
    while read(&mut d, &mut buf, &mut size)? {
        dst.write_all(&buf[0..size])?;
    }

    Ok(())
}
