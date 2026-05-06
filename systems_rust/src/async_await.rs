use tokio::time::{sleep, Duration};


// The `await` keyword is used to pause the execution of the `work` function until the `sleep` future is complete. 
// This allows other tasks to run while waiting, making it an efficient way to handle asynchronous operations without blocking the thread.

async fn task1() {
    println!("task1 start");
    sleep(Duration::from_secs(2)).await;
    println!("task1 end");
}

async fn task2() {
    println!("task2 start");
    sleep(Duration::from_secs(1)).await;
    println!("task2 end");
}

#[tokio::main]
pub async fn main() {
    tokio::spawn(task1());
    tokio::spawn(task2());

    sleep(Duration::from_secs(3)).await;
}
// async - Pause task → run something else → come back later
// Cooperative → YOU decide when to yield (.await)

// Think of Tokio as:
// A mini OS scheduler in user space

// It manages:
// 1. Task scheduling
// Which task runs next
// 2. Polling
// Checking if a task is ready
// 3. Wakeups
// “Hey, your I/O is done, resume!”

// Tokio repeatedly does:

// for each task:
//     poll(task)