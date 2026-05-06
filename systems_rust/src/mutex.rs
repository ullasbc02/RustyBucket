use std::sync::{Arc, Mutex};
use std::thread;
pub fn mutex(){
    println!();
    println!("Mutex example");
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
    
}
//Heap:
// +----------------------+
// | Mutex                |
// |   value = 0          |
// |   locked = false     |
// +----------------------+

// Arc Control Block:
// +----------------------+
// | ref_count = 1        |
// | pointer -> Mutex     |
// +----------------------+