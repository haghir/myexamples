trait Foo<'a> {
    fn get_value1(&self) -> &'a str;
}

struct FooImpl<'a> {
    value1: &'a str
}

impl<'a> Foo<'a> for FooImpl<'a> {
    fn get_value1(&self) -> &'a str {
        self.value1
    }
}

fn g<'a>(foo: &dyn Foo<'a>) {
    println!("{}", foo.get_value1());
}

fn f<'a>(foo: &impl Foo<'a>) {
    g(foo);
}

fn main() {
    let value1;
    {
        let str = String::from("abc");
        let foo = FooImpl {value1: str.as_str()};
        value1 = foo.get_value1();
        println!("{}", value1);

        f(&foo);
    }
    //println!("{}", value1); // NGq
}
