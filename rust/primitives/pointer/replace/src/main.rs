use std::sync::Mutex;

#[derive(Debug)]
struct Baz {
    value: i32,
}

#[derive(Debug)]
struct Bar {
    baz: Option<Box<Baz>>,
}

#[derive(Debug)]
struct Foo {
    bar: Mutex<Bar>,
}

fn main() {
    let foo = Foo {
        bar: Mutex::new(Bar {
            baz: Some(Box::new(Baz {
                value: 123,
            }))
        }),
    };

    {
        let mut bar = foo.bar.lock().unwrap();
        let loc1 = format!("{:p}", &bar.baz.as_ref().unwrap().value);

        let mut bar_orig = std::mem::replace(&mut *bar, Bar{ baz: None });
        let loc2 = format!("{:p}", &bar_orig.baz.as_ref().unwrap().value);

        assert_eq!(loc1, loc2);
    }
}
