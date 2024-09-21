use tokio::runtime::Runtime;

async fn my_async_function2() -> u32 {
    42
}

async fn my_async_function() -> u32 {
    my_async_function2().await
}

fn main() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(my_async_function());
    println!("Result: {}", result);
}
