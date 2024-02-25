// Import functions defined in sibling modules.
use super::foo2::foo2;
use super::fnfoofoo;

pub fn foo3(caller: &'static str) {
    foo2("foo3");
    fnfoofoo("foo3");
    println!("foo3 called from {}", caller);
}