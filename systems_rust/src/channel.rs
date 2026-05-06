use std::sync::mpsc; // Multiple Producer, Single Consumer
use std::thread;
//A channel is a way for threads to communicate by sending data instead of sharing memory.
pub fn main(){
    println!();
    println!("Channel example");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello").unwrap(); // ownership of `tx` is moved to the new thread, allowing it to send messages to the main thread. The `send` method is used to transmit data through the channel, and `unwrap` is called to handle any potential errors that may occur during sending.
    });

    let received = rx.recv().unwrap();

    println!("{}", received);
}
// Thread A ───send──▶ [ Channel ] ───recv──▶ Thread B
// Do not communicate by sharing memory; share memory by communicating.
// Main thread → calls recv() → waits
// Worker thread → sends "hello"
// Main thread → wakes up → gets value


// Mutex problems
// Multiple threads → same variable → need locks
// → risk of:
//    - deadlocks
//    - race conditions
//    - complex reasoning

// Channels solve these problems by allowing threads to communicate through message passing, eliminating the need for shared mutable state and locks. Each thread can send messages to others without worrying about synchronization issues, as the channel handles the communication safely and efficiently.
// Thread 1 ──┐
//            ├──▶ Channel ───▶ Receiver
// Thread 2 ──┘

// | Feature        | Mutex / RwLock | Channel            |
// | -------------- | -------------- | ------------------ |
// | Model          | Shared memory  | Message passing    |
// | Sync needed    | Yes (locks)    | No locks needed    |
// | Complexity     | Higher         | Lower              |
// | Data ownership | Shared         | Transferred        |
// | Use case       | Shared state   | Task communication |
