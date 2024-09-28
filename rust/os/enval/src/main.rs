use std::env;

fn main() {
    println!("{:?}", env::var("USER"));
    println!("{:?}", env::var("USERx"));
    println!("{:?}", env::var_os("USER"));
    println!("{:?}", env::var_os("USERx"));
}
