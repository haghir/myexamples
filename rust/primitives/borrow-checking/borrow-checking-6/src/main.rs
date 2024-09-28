struct Foo;

struct Bar<'a> {
    foo: &'a Foo,
}

struct Baz<'a> {
    bar: &'a Bar<'a>,
}

struct BarWrapper<'a> {
    bar: Bar<'a>,
}

struct BazWrapper<'a> {
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

impl<'a> BarWrapper<'a> {
    fn new(bar: Bar<'a>) -> Self {
        BarWrapper { bar }
    }

    fn new_baz(&self) -> BazWrapper {
        BazWrapper::new(self.bar.new_baz())
    }
}

impl<'a> BazWrapper<'a> {
    fn new(baz: Baz<'a>) -> Self {
        BazWrapper { baz }
    }

    fn print(&self) {
        self.baz.print();
    }
}

fn main() {
    let foo = Foo {};
    let bar = BarWrapper::new(foo.new_bar());
    let baz = bar.new_baz();
    baz.print();
}
