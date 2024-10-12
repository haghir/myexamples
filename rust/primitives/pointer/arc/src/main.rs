use std::sync::Arc;

fn bar(o: String, p: Arc<String>, q: Arc<Box<String>>) {
    println!("bar strong count: {}", Arc::strong_count(&p));
    println!("bar weak count: {}", Arc::weak_count(&p));
    println!("{:p}, {:p}, {:p}", &o, p, q);
}

fn foo(o: String, p: Arc<String>, q: Arc<Box<String>>) {
    println!("foo strong count: {}", Arc::strong_count(&p));
    println!("foo weak count: {}", Arc::weak_count(&p));
    println!("{:p}, {:p}, {:p}", &o, p, q);

    bar(o, p.clone(), q.clone());

    println!("foo strong count: {}", Arc::strong_count(&p));
    println!("foo weak count: {}", Arc::weak_count(&p));
}

fn main() {
    let o = "text value".to_string();
    let p = Arc::new(String::from("text value"));
    let q = Arc::new(Box::new(String::from("text value")));
    println!("strong count: {}", Arc::strong_count(&p));
    println!("weak count: {}", Arc::weak_count(&p));
    println!("{:p}, {:p}, {:p}", &o, p, q);

    foo(o, p.clone(), q.clone());

    println!("strong count: {}", Arc::strong_count(&p));
    println!("weak count: {}", Arc::weak_count(&p));
}
