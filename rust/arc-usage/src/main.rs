use std::sync::Arc;

fn bar(p: Arc<String>) {
    println!("bar strong count: {}", Arc::strong_count(&p));
    println!("bar weak count: {}", Arc::weak_count(&p));
}

fn foo(p: Arc<String>) {
    println!("foo strong count: {}", Arc::strong_count(&p));
    println!("foo weak count: {}", Arc::weak_count(&p));

    bar(p.clone());

    println!("foo strong count: {}", Arc::strong_count(&p));
    println!("foo weak count: {}", Arc::weak_count(&p));
}

fn main() {
    let p = Arc::new(String::from("text value"));
    println!("strong count: {}", Arc::strong_count(&p));
    println!("weak count: {}", Arc::weak_count(&p));

    foo(p.clone());

    println!("strong count: {}", Arc::strong_count(&p));
    println!("weak count: {}", Arc::weak_count(&p));
}
