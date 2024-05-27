struct Foo;

struct Bar<'a> {
    foo: &'a Foo,
}

struct Baz<'a> {
    bar: &'a Bar<'a>,
}

struct BarBaz<'a> {
    bar: Bar<'a>,
    baz: Baz<'a>,
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

fn f<'a>(foo: &'a Foo) -> BarBaz {
    let bar = foo.new_bar();
    let baz = bar.new_baz();

    // Some errors will occur here.
    BarBaz {
        bar: bar,
        baz: baz,
    }
}

fn main() {
    let foo = Foo {};
    let barbaz = f(&foo);
    barbaz.baz.print();
}
