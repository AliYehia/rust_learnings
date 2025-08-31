use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let other_tx = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("message"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            if let Err(e) = tx.send(val) {
                println!("Failed to send value: {}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let more_vals = vec![
            String::from("more"),
            String::from("data"),
            String::from("from"),
            String::from("other"),
            String::from("transmitter"),
        ];

        for val in more_vals {
            if let Err(e) = other_tx.send(val) {
                println!("Failed to send value from other transmitter: {}", e)
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("Got: {}", recieved);
    }

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}