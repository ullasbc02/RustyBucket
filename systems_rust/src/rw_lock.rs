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


/*
Yes. This is very similar to `Mutex`, but `RwLock` has one extra idea:

```text
Mutex  = only one thread can access data at a time

RwLock = many readers OR one writer
```

So:

```rust
Arc<RwLock<i32>>
```

means:

Arc    = many threads can own the same data
RwLock = many can read, but only one can write


---

# Step 1: Initial Allocation

```rust
let data = Arc::new(RwLock::new(5));
```

Memory looks like:

```text
Main Stack

data
 │
 ▼
Arc Control Block
 ref_count = 1
 pointer ──────────────┐
                       ▼
                   RwLock
                   value = 5
                   readers = 0
                   writer = false
```

---

# Step 2: Loop Starts

```rust
for i in 0..3 {
    let data_clone = Arc::clone(&data);
```

Each loop creates one new `Arc` pointer.

The value `5` is **not copied**.

After 3 clones:

```text
Main data ───────┐
Thread 0 Arc ────┤
Thread 1 Arc ────┤──> same RwLock(value = 5)
Thread 2 Arc ────┘

Arc ref_count = 4
```

One owner in `main`, three owners inside reader threads.

---

# Step 3: Reader Threads Start

Inside each thread:

```rust
let num = data_clone.read().unwrap();
println!("Read:{} {}", i, *num);
```

`read()` tries to acquire a **read lock**.

If no writer is active:

```text
writer = false
```

then readers are allowed.

So multiple readers can enter together.

Example:

```text
Thread 0 calls read()
readers = 1

Thread 1 calls read()
readers = 2

Thread 2 calls read()
readers = 3
```

All can read at the same time.

Memory:

```text
RwLock
value = 5
readers = 3
writer = false
```

Each reader gets read-only access:

```rust
&i32
```

Not mutable access.

So this is allowed:

```rust
println!("{}", *num);
```

But this is not allowed:

```rust
*num += 1; // ❌ cannot mutate through read lock
```

---

# Step 4: Main Tries To Write

After spawning reader threads, main does:

```rust
{
    let mut write_num = data.write().unwrap();
    *write_num += 10;
    println!("Write: {}", *write_num);
}
```

`write()` asks:

```text
Can I get exclusive access?
```

It can only continue when:

```text
readers = 0
writer = false
```

So if readers are still active:

```text
readers = 3
writer = false
```

main blocks here:

```rust
let mut write_num = data.write().unwrap();
```

It waits.

---

# Step 5: Readers Finish

When a reader thread finishes:

```rust
let num = data_clone.read().unwrap();

println!("Read:{} {}", i, *num);
```

At the end of the thread, `num` goes out of scope.

That releases the read lock.

So:

```text
readers = 3 → 2 → 1 → 0
```

Once all readers are done:

```text
readers = 0
writer = false
```

Now main can acquire write lock.

---

# Step 6: Writer Gets Exclusive Access

Now this line succeeds:

```rust
let mut write_num = data.write().unwrap();
```

RwLock becomes:

```text
RwLock
value = 5
readers = 0
writer = true
```

Main gets mutable access:

```rust
&mut i32
```

Then:

```rust
*write_num += 10;
```

Value changes:

```text
5 → 15
```

Then prints:

```text
Write: 15
```

---

# Step 7: Writer Lock Releases

Because of this block:

```rust
{
    let mut write_num = data.write().unwrap();
    *write_num += 10;
    println!("Write: {}", *write_num);
}
```

When the block ends, `write_num` is dropped.

That releases the write lock.

```text
RwLock
value = 15
readers = 0
writer = false
```

This `{ }` block is important because it controls when the write lock is released.

---

# Step 8: join()

```rust
for handle in handles {
    handle.join().unwrap();
}
```

Main waits until all reader threads finish.

If they already finished, `join()` returns immediately.

If some are still running, main waits for them.

---

# Important: Output Can Change

Because threads are scheduled non-deterministically, output may be different.

Case 1: Readers run first

```text
Read:0 5
Read:1 5
Read:2 5
Write: 15
```

Case 2: Main gets write lock before readers run

```text
Write: 15
Read:0 15
Read:1 15
Read:2 15
```

Case 3: Some readers run before write, some after

```text
Read:0 5
Write: 15
Read:1 15
Read:2 15
```

But this part is always safe:

```text
No reader can read while writer is writing.
No writer can write while readers are reading.
Only one writer can write at a time.
```

---

# Mutex vs RwLock

```text
Mutex:
Only one thread at a time.
Reader? one at a time.
Writer? one at a time.
```

```text
RwLock:
Many readers at the same time.
Only one writer at a time.
Writer needs exclusive access.
```

So this:

```rust
Arc<Mutex<i32>>
```

means:

```text
Shared ownership + one-at-a-time access
```

This:

```rust
Arc<RwLock<i32>>
```

means:

```text
Shared ownership + many-readers / one-writer access
```

---

# Best Mental Model

```text
Arc = many people have the key to the building

Mutex = only one person can enter the room

RwLock = many people can enter to read,
         but if someone wants to write,
         everyone else must leave first
```

In your code:

```text
3 reader threads may read 5

main writer adds 10

after write, future readers see 15
```

Final idea:


RwLock is useful when reads are frequent and writes are rare.


*/