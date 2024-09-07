use std::path::{Path, PathBuf};

fn main() {
    let path: PathBuf = ["/", "foo", "bar"].iter().collect();
    println!("{:?}", path);
}