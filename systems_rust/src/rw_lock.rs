use std::sync::{Arc, RwLock};
use std::thread;

pub fn main() {
    println!();
    println!("RwLock example");
    let data = Arc::new(RwLock::new(5));

    let mut handles = vec![];

    // Threads are scheduled non-deterministically, so execution order may vary each run.
    // Here, the writer updates the value to 15, while reader threads may observe either 5 or 15.
    for i in 0..3 {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let num = data_clone.read().unwrap();

            println!("Read:{} {}", i, *num);
        });

        handles.push(handle);
    }

    {
        // only execute when readers = 0, writer = false
        let mut write_num = data.write().unwrap(); // Acquires a write lock, blocking until all readers have released their locks.


        *write_num += 10;
        println!("Write: {}", *write_num);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
// Stack:
// data ───────────────┐
//                     ↓
// Heap:
// Arc Control Block:
//   ref_count = 1
//   pointer ───────────────→ RwLock

// RwLock:
//   value = 5
//   readers = 0
//   writer = false