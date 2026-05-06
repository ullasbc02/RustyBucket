use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Multi-threaded Counter");

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            println!("Thread {} started", i);

            let mut num = counter_clone.lock().unwrap();
            *num += 1;

            println!("Thread {} incremented", i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = *counter.lock().unwrap();
    println!("Final count: {}", final_count);
}