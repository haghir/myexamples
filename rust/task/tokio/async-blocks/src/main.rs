use std::time::Duration;

use tokio::{task::JoinSet, time::sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut join_set = JoinSet::new();

    join_set.spawn(async move {
        println!("async 1 started");
        sleep(Duration::from_secs(5)).await;
        println!("async 1 running");
        Ok::<&'static str, Box<dyn std::error::Error + Sync + Send>>("async 1 completed")
    });

    join_set.spawn(async move {
        println!("async 2 started");
        sleep(Duration::from_secs(10)).await;
        println!("async 2 running");
        Err("Something went wrong on async 2".into())
    });

    join_set.spawn(async move {
        println!("async 3 started");
        sleep(Duration::from_secs(15)).await;
        println!("async 3 running");
        Ok("async 3 completed")
    });

    join_set.spawn(async move {
        println!("async 4 started");
        sleep(Duration::from_secs(1)).await;
        println!("async 4 running");
        panic!("panic on async 4")
    });

    while let Some(ret) = join_set.join_next().await {
        match ret {
            Ok(ret) => match ret {
                Ok(message) => println!("OK: {}", message),
                Err(err) => println!("Err: {}", err),
            },
            Err(err) => println!("failed: {}", err),
        }
    }

    Ok(())
}
