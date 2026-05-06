use std::sync::Arc;
use std::thread;
mod r#mutex;
mod r#rw_lock;
mod r#channel;
mod r#async_await;
fn main() {
    //println!("Hello, world!");
    let data = vec![1,2,3];

    // `thread::spawn` creates a new OS-level thread and executes the code 
    // inside the closure independently of the main thread. By using `move`, 
    // ownership of `data` is permanently transferred to the new thread, 
    // causing the compiler to block any further access in `main` to prevent memory safety issues.
    thread::spawn(move || {
        println!("{:?}", data);
    });

    //println!("{:?}", data); // ❌ moved


    // Many owners can safely share data across threads using 
    // `Arc` (Atomic Reference Counted) and `Mutex` (Mutual Exclusion).
    // `Arc` allows multiple threads to have shared ownership of the same data,
    // while `Mutex` ensures that only one thread can access the data at a time, preventing race conditions.

    let numbers = Arc::new(vec![1, 2, 3]);

    let mut handles = vec![];

    for i in 0..3 {
        let numbers_clone = Arc::clone(&numbers);

        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, numbers_clone);
        });

        handles.push(handle);
    }
    // println!("Main thread: {:?}", handles);
    for handle in handles {
        handle.join().unwrap();
    }

    r#mutex::mutex();
    r#rw_lock::main();
    r#channel::main();
    r#async_await::main();
}
// To explain the flow with the stack and heap, we will look at how the addresses move as the code executes.

// ### **Step 1: The Allocation (`Arc::new`)**
// When `main` starts, it allocates the vector on the heap. But because of `Arc`, it also creates a **Control Block**.

// * **Heap Address `0x222`**: Contains the actual data `[1, 2, 3]`.
// * **Heap Address `0x111`**: The Control Block. It contains the **Strong Count (currently 1)** and a pointer to `0x222`.
// * **Stack (`main`)**: The variable `numbers` is created. Its value is `0x111`.



// ---

// ### **Step 2: Inside the Loop (The `Clone`)**
// As the loop runs for the first time ($i=0$):
// 1.  **`numbers_clone`** is created on the **Main Stack**.
// 2.  It copies the value `0x111` from `numbers`.
// 3.  It sends a signal to the Control Block at `0x111` to increment the count.

// * **Heap `0x111`**: Strong Count is now **2**.
// * **Stack (`main`)**: Now has two variables, `numbers` and `numbers_clone`, both holding `0x111`.

// ---

// ### **Step 3: The `spawn` and `move`**
// This is where the stack "pointing" changes location. When `thread::spawn(move || ...)` is called:
// 1.  A new **Thread 0 Stack** is created by the OS.
// 2.  The variable `numbers_clone` is **popped** off the Main Stack and **pushed** onto the Thread 0 Stack.
// 3.  The value stays `0x111`.

// * **Stack (`main`)**: `numbers_clone` is gone. `handle` (a thread receipt) is added to the `handles` vector.
// * **Stack (`Thread 0`)**: Now contains `numbers_clone` (value: `0x111`).



// ---

// ### **Step 4: Managing the Receipts (`handles.push`)**
// ```rust
// handles.push(handle);
// ```
// The loop finishes 3 times. At the end of the loop, the **Main Stack** looks like this:
// * `numbers`: `0x111`
// * `handles`: A list containing three receipts: `[Receipt_0, Receipt_1, Receipt_2]`.

// Meanwhile, there are **three separate Thread Stacks** ($0, 1,$ and $2$), each holding a copy of the address `0x111`. The **Strong Count** at `0x111` is now **4** (Main + 3 Threads).

// ---

// ### **Step 5: The Synchronization (`handle.join`)**
// ```rust
// for handle in handles {
//     handle.join().unwrap();
// }
// ```
// 1.  **Main** reaches `join()` for Thread 0. The Main Thread "pauses" its stack execution.
// 2.  **Thread 0** finishes its `println!`. It reaches its closing brace `}`.
// 3.  **Automatic Drop:** Thread 0's stack is cleared. Its `numbers_clone` disappears.
// 4.  **Signal:** As it disappears, it tells the Control Block at `0x111`: "Decrement count." (Count becomes **3**).
// 5.  **Main** wakes up because Thread 0 is done, and moves to the next `join()`.



// ---

// ### **Step 6: The Final Cleanup**
// After all 3 `join()` calls are finished:
// * All worker thread stacks are gone.
// * The Strong Count at `0x111` is back to **1** (only `main` still has a key).
// * `main` reaches its final `}`.
// * **The Final Drop:** `numbers` is dropped from the Main Stack.
// * The count at `0x111` hits **0**.
// * **The Sweep:** Because the count is 0, the program deletes the Control Block at `0x111` **and** the actual vector at `0x222`.

// ### **Summary of Pointer Flow**
// * **Allocation:** `main` points to `0x111`.
// * **Clone:** `main` creates a temporary pointer to `0x111`.
// * **Move:** That temporary pointer slides from the **Main Stack** to the **Thread Stack**.
// * **Join:** `main` waits for the **Thread Stack** to disappear so it can safely move to the exit.