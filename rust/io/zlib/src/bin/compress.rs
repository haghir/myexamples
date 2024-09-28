use std::env;
use std::io::prelude::*;
use std::fs::File;
use flate2::Compression;
use flate2::write::ZlibEncoder;

fn read(file: &mut impl Read, buf: &mut [u8], size: &mut usize) -> std::io::Result<bool> {
    *size = file.read(buf)?;
    Ok(*size > 0)
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Illegal arguments.");
    }

    let mut src = File::open(args[1].as_str())?;
    let dst = File::create(args[2].as_str())?;
    let mut e = ZlibEncoder::new(dst, Compression::default());

    let mut buf: [u8; 1024] = [0; 1024];
    let mut size = 0;
    while read(&mut src, &mut buf, &mut size)? {
        e.write_all(&buf[0..size])?;
    }
    e.finish()?;

    Ok(())
}
