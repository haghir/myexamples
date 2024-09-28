use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() -> std::thread::Result<()> {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread: {}", i);
            sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("main: {}", i);
        sleep(Duration::from_millis(1));
    }

    handle.join()
}
