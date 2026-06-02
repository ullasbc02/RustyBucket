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

/* 

```rust
Arc<Mutex<i32>>
```

Think of it as:

```text
Arc   = Who owns the house key?
Mutex = Who is allowed inside the room right now?
```

---

# Step 1: Initial Allocation

```rust
let counter = Arc::new(Mutex::new(0));
```

Heap:

```text
Heap
┌─────────────────────┐
│ Mutex               │
│ value = 0           │
│ locked = false      │
└─────────────────────┘
        ▲
        │
┌─────────────────────┐
│ Arc Control Block   │
│ ref_count = 1       │
│ pointer ────────────┘
└─────────────────────┘
```

Main stack:

```text
Main Stack

counter
   │
   ▼
Arc Control Block
```

---

# Step 2: First Loop Iteration

```rust
let counter_clone = Arc::clone(&counter);
```

Only Arc is cloned.

The integer is NOT copied.

Heap:

```text
Arc Control Block
ref_count = 2
```

Stack:

```text
Main Stack

counter --------┐
counter_clone --┘
```

Both point to same mutex.

---

# Step 3: Spawn Thread

```rust
thread::spawn(move || {
```

`counter_clone` moves into thread.

Main:

```text
counter
```

Thread 0:

```text
counter_clone
```

Both still point to same mutex.

---

# After 5 Iterations

You have:

```text
Main Stack

counter
```

Thread 0:

```text
counter_clone
```

Thread 1:

```text
counter_clone
```

Thread 2:

```text
counter_clone
```

Thread 3:

```text
counter_clone
```

Thread 4:

```text
counter_clone
```

All point to SAME mutex.

```text
Arc ref_count = 6
```

(1 main + 5 threads)

---

# Step 4: All Threads Start Running

Suppose Thread 0 arrives first.

```rust
let mut num = counter_clone.lock().unwrap();
```

Mutex state changes:

```text
Before:

value = 0
locked = false

After:

value = 0
locked = true
owner = Thread 0
```

---

# What Happens To Other Threads?

Suppose Thread 1 arrives:

```rust
counter_clone.lock()
```

It sees:

```text
locked = true
```

So it waits.

```text
Thread 1 → BLOCKED
```

Thread 2:

```text
BLOCKED
```

Thread 3:

```text
BLOCKED
```

Thread 4:

```text
BLOCKED
```

Only one thread can enter.

---

# Step 5: Thread 0 Gets Mutable Reference

```rust
let mut num = counter_clone.lock().unwrap();
```

Now:

```text
num
 ↓
value = 0
```

Rust internally gives:

```rust
&mut i32
```

to Thread 0.

This is why mutation is safe.

No other thread can get a mutable reference simultaneously.

---

# Step 6: Increment

```rust
*num += 1;
```

Before:

```text
value = 0
```

After:

```text
value = 1
```

Heap:

```text
Mutex
value = 1
locked = true
```

---

# Step 7: Lock Guard Drops

End of closure:

```rust
{
    let mut num = lock();
    *num += 1;
}
```

`num` goes out of scope.

Rust automatically drops the lock guard.

Equivalent to:

```rust
unlock(mutex);
```

Now:

```text
value = 1
locked = false
```

Thread 1 can proceed.

---

# Step 8: Thread 1 Wakes Up

Thread 1 was sleeping.

Now it acquires lock:

```text
value = 1
locked = true
owner = Thread 1
```

Gets:

```rust
&mut i32
```

Then:

```rust
*num += 1;
```

Result:

```text
value = 2
```

Releases lock.

---

# Step 9: Repeat

Thread 2:

```text
2 → 3
```

Thread 3:

```text
3 → 4
```

Thread 4:

```text
4 → 5
```

Final heap:

```text
Mutex
value = 5
locked = false
```

---

# Step 10: join()

```rust
for handle in handles {
    handle.join().unwrap();
}
```

Main waits for all threads.

As each thread exits:

```text
Arc ref_count
6 → 5
5 → 4
4 → 3
3 → 2
2 → 1
```

Only main's `counter` remains.

---

# Step 11: Final Print

```rust
println!("Final count: {}", *counter.lock().unwrap());
```

Main acquires lock:

```text
locked = true
```

Reads:

```text
value = 5
```

Prints:

```text
Final count: 5
```

Then lock releases.

---

# Why Can't We Do This Without Mutex?

Imagine:

```rust
Arc<i32>
```

and all threads do:

```rust
counter += 1;
```

Suppose value is 0.

Thread 0 reads:

```text
0
```

Thread 1 reads:

```text
0
```

Thread 2 reads:

```text
0
```

All compute:

```text
0 + 1
```

All write:

```text
1
```

Final value:

```text
1
```

instead of:

```text
5
```

This is called a **race condition**.

Mutex prevents this by forcing:

```text
Thread 0: 0 → 1
Thread 1: 1 → 2
Thread 2: 2 → 3
Thread 3: 3 → 4
Thread 4: 4 → 5
```

one at a time.

---

### The most important interview takeaway

```rust
Arc<Mutex<T>>
```

solves two different problems:

```text
Arc
 └── Shared ownership across threads

Mutex
 └── Exclusive mutable access to data
```

So remember:

```text
Arc = Who owns the data?

Mutex = Who can modify the data right now?
```

Your code is the canonical Rust pattern for a shared counter across multiple threads. 
*/