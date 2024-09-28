trait Foo {
    fn foo(&self);
}

struct Bar {
    message: String,
}

struct Baz {
    message: String,
}

impl Foo for Bar {
    fn foo(&self) {
        println!("Bar.foo() message: {}", self.message);
    }
}

impl Foo for Baz {
    fn foo(&self) {
        println!("Baz.foo() message: {}", self.message);
    }
}

fn f(foo: &impl Foo) {
    foo.foo();
}

fn main() {
    let bar = Bar { message: String::from("Bar") };
    let baz = Baz { message: String::from("Baz") };

    f(&bar);
    f(&baz);
}
