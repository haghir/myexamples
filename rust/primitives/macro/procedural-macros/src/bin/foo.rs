use procedural_macros::{DeriveMacro, attr_macro, fn_macro};

#[derive(DeriveMacro)]
struct Foo {
    value: i32,
}

#[attr_macro(message1)]
struct Bar1 {
}

#[attr_macro(message2)]
struct Bar2 {
    fox: &'static str,
    dog: &'static str,
}

#[attr_macro(message3)]
struct Bar3 {
    quick: i32,
    lazy: i32,
}

fn_macro! {
    fn baz() -> i32 {
        123
    }
}

fn main() {
    let foo = Foo { value: 123 };
    let bar1 = Bar1 {};
    let bar2 = Bar2 { fox: "elephant", dog: "giraffe" };
    let bar3 = Bar3 { quick: 20, lazy: 100 };

    println!("{}", foo.get_value());
    println!("{}", bar1.format());
    println!("{}", bar2.format());
    println!("{}", bar3.format());
    println!("{}", double_baz());
}
