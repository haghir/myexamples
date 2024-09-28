use std::{io, fs, path::Path};

fn ls(item: &Path) -> io::Result<()> {
    if item.is_dir() {
        for entry in fs::read_dir(item)? {
            let entry = entry?;
            let path = entry.path();
            ls(&path)?;
        }
    } else if let Some(path_str) = item.to_str() {
        println!("{}", path_str);
    }
    Ok(())
}

fn main() {
    let path = Path::new(".");
    ls(&path).unwrap();
}
