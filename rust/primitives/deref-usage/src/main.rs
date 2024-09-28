use std::ops::Deref;

#[derive(Debug)]
struct Foo {
    value: i64,
}

impl Foo {
    fn new(value: i64) -> Self {
        Self {
            value,
        }
    }
}

#[derive(Debug)]
struct FooBox {
    foo: Foo,
}

impl FooBox {
    fn new(foo: Foo) -> FooBox {
        Self {
            foo,
        }
    }
}

impl Deref for FooBox {
    type Target = Foo;

    fn deref(&self) -> &Self::Target {
        &self.foo
    }
}

#[derive(Debug)]
struct FooBoxBox {
    foo: FooBox,
}

impl FooBoxBox {
    fn new(foo: FooBox) -> Self {
        Self {
            foo,
        }
    }
}

impl Deref for FooBoxBox {
    type Target = FooBox;

    fn deref(&self) -> &Self::Target {
        &self.foo
    }
}

fn main() {
    let foo = FooBoxBox::new(FooBox::new(Foo::new(123)));

    println!("{:?}", foo);
    println!("{:?}", *foo);
    println!("{:?}", **foo);

    println!("{}", foo.value);
    println!("{}", (*foo).value);
    println!("{}", (**foo).value);
}
