use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle = tokio::task::spawn(async {
        panic!("Something went wrong");
    });
    sleep(Duration::from_secs(1)).await;

    println!("before awaiting");

    sleep(Duration::from_secs(1)).await;
    handle.await?;

    println!("after awaiting");

    Ok(())
}
