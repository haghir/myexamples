struct Foo;
struct Bar;
struct Baz;

impl Foo {
    fn new_bar(&self) -> Bar {
        Bar {}
    }
}

impl Bar {
    fn new_baz(&self) -> Baz {
        Baz {}
    }
}

fn main() {
    let foo = Foo {};
    let bar = foo.new_bar();
    let _baz = bar.new_baz();
}
