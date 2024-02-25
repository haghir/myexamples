use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    if let Ok(_) = file.read_to_string(&mut s) {
        println!("{}", s);
    }
}
