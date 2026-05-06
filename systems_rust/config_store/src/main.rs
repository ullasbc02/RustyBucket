use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Config Store with RwLock");

    let config = Arc::new(RwLock::new(String::from("version_1")));

    let mut handles = vec![];

    // Readers
    for i in 0..3 {
        let config_clone = Arc::clone(&config);

        let handle = thread::spawn(move || {
            println!("Reader {} waiting", i);

            let data = config_clone.read().unwrap();

            println!("Reader {} read config: {}", i, *data);

            thread::sleep(Duration::from_secs(2));

            println!("Reader {} done", i);
        });

        handles.push(handle);
    }

    // Give readers time to acquire lock
    thread::sleep(Duration::from_millis(500));

    // Writer
    {
        println!("Writer waiting for write lock");

        let mut write_data = config.write().unwrap();

        println!("Writer acquired lock");

        *write_data = String::from("version_2");

        println!("Writer updated config");
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final config: {}", *config.read().unwrap());
}