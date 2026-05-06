use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Producer Consumer Queue");

    let (tx, rx) = mpsc::channel();

    // Producer thread
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            let log = format!("log {}", i);

            println!("Producing: {}", log);

            tx.send(log).unwrap();

            thread::sleep(Duration::from_secs(1));
        }

        println!("Producer finished");
    });

    // Consumer thread
    let consumer = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(message) => {
                    println!("Consumed: {}", message);
                }

                Err(_) => {
                    println!("Channel closed");
                    break;
                }
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}