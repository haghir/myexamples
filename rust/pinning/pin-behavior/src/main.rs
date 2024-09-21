use std::marker::PhantomPinned;
use std::mem::replace;

struct Foo {
    x: i32,
}

struct Bar {
    x: i32,
    _pin: PhantomPinned,
}

fn main() {
    let mut foo1 = Foo { x: 123 };
    let foo1 = &mut foo1;
    println!("foo1: {:p}, {}", foo1, foo1.x);
    let foo2 = replace(foo1, Foo { x: 456 });
    println!("foo1: {:p}, {}", foo1, foo1.x);
    println!("foo2: {:p}, {}", &foo2, foo2.x);

    let mut foo3 = Box::new(Foo { x: 123 });
    println!("foo3: {:p}, {}", foo3, foo3.x);
    let foo4 = replace(&mut *foo3, Foo { x: 456 });
    println!("foo3: {:p}, {}", foo3, foo3.x);
    println!("foo4: {:p}, {}", &foo4, foo4.x);

    let mut bar1 = Bar { x: 123, _pin: PhantomPinned };
    let bar1 = &mut bar1;
    println!("bar1: {:p}, {}", bar1, bar1.x);
    let bar2 = replace(bar1, Bar { x: 456, _pin: PhantomPinned });
    println!("bar1: {:p}, {}", bar1, bar1.x);
    println!("bar2: {:p}, {}", &bar2, bar2.x);

    /*
    let mut bar3 = Box::pin(Bar { x: 123, _pin: PhantomPinned });
    println!("{:p}, {}", bar3, bar3.x);
    // error[E0596]: cannot borrow data in dereference of `Pin<Box<Bar>>` as mutable
    let bar4 = replace(&mut *bar3, Bar { x: 456, _pin: PhantomPinned });
    println!("{:p}, {}", bar3, bar3.x);
    println!("{:p}, {}", &bar4, bar4.x);
     */
}
