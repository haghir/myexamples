struct Foo {
    current: i32
}

impl Foo {
    fn new() -> Self {
        Foo {
            current: 0
        }
    }
}

impl Iterator for Foo {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 10 {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    let foo = Foo::new();
    for x in foo {
        println!("{}", x);
    }
}
