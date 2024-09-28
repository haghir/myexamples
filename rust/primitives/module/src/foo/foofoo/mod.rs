// Expose the module foo2.
pub mod foo2;

// Expose the function foo3, but keep the module foo3 private from super modules.
mod foo3;
pub use foo3::foo3;

pub fn fnfoofoo(caller: &'static str) {
    println!("fnfoofoo called from {}", caller);
}