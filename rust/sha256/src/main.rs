use std::env;
use std::io::prelude::*;
use std::fs::File;
use sha2::{Sha256, Digest};

fn read(file: &mut File, buf: &mut [u8], size: &mut usize) -> std::io::Result<bool> {
    *size = file.read(buf)?;
    Ok(*size > 0)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args[1].as_str();
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf: [u8; 1024] = [0; 1024];
    let mut size = 0;
    while read(&mut file, &mut buf, &mut size)? {
        hasher.update(&buf[0..size])
    }

    let hash = hex::encode(hasher.finalize());
    println!("{} *{}", hash, path);

    Ok(())
}
