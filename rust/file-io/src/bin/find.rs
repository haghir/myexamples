use std::{convert::TryInto, fs::read_dir, path::Path};

fn indent(depth: i64) -> String {
    let mut ret = String::new();
    for _ in 0..depth {
        ret.push_str("  ");
    }
    ret
}

fn scan<P>(dir_path: P, depth: i64)
where
    P: AsRef<Path>,
{
    let dir_path = dir_path.as_ref();
    for entry in read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ftype = entry.file_type().unwrap();

        println!(
            "{}{}: is_dir = {}, is_file = {}, is_symlink = {}",
            indent(depth),
            path.clone().into_os_string().into_string().unwrap(),
            ftype.is_dir(),
            ftype.is_file(),
            ftype.is_symlink()
        );

        if ftype.is_dir() {
            scan(&path, depth + 1)
        }
    }
}

fn main() {
    scan("test", 0);
}
