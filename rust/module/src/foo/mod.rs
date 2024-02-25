// Expose the module foofoo to super modules.
pub mod foofoo;

// Expose the module foo1 to super modules,
// and expose the function foo1 as foo1a.
pub mod foo1;
pub use foo1::foo1 as foo1a;
