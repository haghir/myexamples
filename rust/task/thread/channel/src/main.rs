use crossbeam_channel::{unbounded, Sender};
use std::{
    collections::LinkedList,
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

enum Message {
    Text(String),
    Term,
}

fn main() {
    let mut senders: Vec<Sender<Message>> = Vec::new();
    let mut handlers: LinkedList<JoinHandle<()>> = LinkedList::new();

    for i in 0..5 {
        let (s, r) = unbounded();
        senders.push(s);

        let handler = spawn(move || {
            let dur = Duration::from_millis(1);
            for message in r {
                match message {
                    Message::Text(message) => {
                        println!("Thread #{} received a message: {}", i, message)
                    }
                    Message::Term => {
                        println!("Thread #{} was terminated.", i);
                        break;
                    }
                }
                sleep(dur);
            }
        });
        handlers.push_back(handler);
    }

    for i in 0..100 {
        if let Err(e) = senders[i % senders.len()].send(Message::Text(format!("message #{}", i))) {
            println!("Failed to send a text message: {}", e);
        }
    }

    // Sender (and Receiver) can be cloned.
    let senders: Vec<Sender<Message>> = senders.iter().map(|x| x.clone()).collect();

    for sender in senders {
        if let Err(e) = sender.send(Message::Term) {
            println!("Failed to send a termination message: {}", e);
        }
    }

    println!("Wait for join.");
    while let Some(handler) = handlers.pop_back() {
        if let Err(_) = handler.join() {
            println!("Failed to join");
        }
    }
}
