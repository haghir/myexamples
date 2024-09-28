//! refs: https://veykril.github.io/tlborm/decl-macros/macros-methodical.html

macro_rules! four {
    () => { 1 + 3 };
    (4 fn ['spang "whammo"] @_@) => { 2 + 2 };
}

macro_rules! foo {
    ($($e:expr),+) => {
        $(
            print!("{}, ", $e);
        )*
        println!();
    };
}

fn main() {
    println!("{}", four!() * four![]);
    println!("{}", four!(4 fn ['spang "whammo"] @_@));
    foo!(123, "456", '7');
    foo![234, "567", '8'];
    foo!{345, "678", '9'};
}
