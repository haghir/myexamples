struct Foo;

struct Bar<'a> {
    foo: &'a Foo,
}

struct Baz<'a> {
    bar: &'a Bar<'a>,
}

impl Foo {
    fn new_bar<'a>(&'a self) -> Bar {
        Bar { foo: self }
    }

    fn print(&self) {
        println!("foo");
    }
}

impl<'a> Bar<'a> {
    fn new_baz(&'a self) -> Baz {
        Baz { bar: self }
    }

    fn print(&self) {
        self.foo.print();
    }
}

impl<'a> Baz<'a> {
    fn print(&self) {
        self.bar.print();
    }
}

fn main() {
    let foo = Foo {};
    let bar = foo.new_bar();
    let baz = bar.new_baz();
    baz.print();
}
