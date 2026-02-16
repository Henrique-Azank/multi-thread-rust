# Shared State

The goal is to safely share mutable state between multiple threads without causing data races or race conditions. This is accomplished using `Arc` (Atomic Reference Counting) for shared ownership and `Mutex` (Mutual Exclusion) for thread-safe access to the shared data.

## Arc (Atomic Reference Counting)

Arc is a thread-safe reference-counting pointer that allows multiple threads to own the same data. It is similar to Rc (Reference Counting), but is designed for concurrent use.

### Code Structure

```rust
let counter = Arc::new(Counter::new());
let counter_clone = Arc::clone(&counter);
```

The implementation of `Arc` consists on:

`Arc::new()` -> Creates a new Arc, allocating the data on the heap and wrapping it;

`Arc::clone()` -> Creates a new reference to the same data, incrementing the reference count atomically. This is not a deep copy, but a cheap reference.

`Drop` -> When an Arc goes out of scope, the reference count is decremented atomically. When the count reaches zero, the data is deallocated.

## Mutex (Mutual Exclusion)

Mutex is a lock that ensures only one thread can access the protected data at a time. It prevents data races by enforcing exclusive access.

The implementation of `Mutex` consists on:

`Mutex<T>` -> A generic wrapper around data of type `T`. Only one thread can lock and access the data at a time;

`.lock()` -> Acquires the lock, returning a `MutexGuard` that holds the lock until it goes out of scope. The method returns a `Result` to handle poisoning (when a thread panics while holding the lock);

`.unwrap()` -> Extracts the `MutexGuard` from the Result. In production code, you should handle potential panics more gracefully.

## Counter Struct

The Counter is a simple data structure that demonstrates how to protect mutable state with a Mutex.

### Code Structure

```rust
struct Counter {
    value: Mutex<usize>,
}
```

The implementation of `Counter` consists on:

`value` -> A `Mutex` wrapping a `usize` counter. The Mutex ensures that only one thread can modify the counter at a time.

### Instantiation

```rust
fn new() -> Self {
    Counter {
        value: Mutex::new(0),
    }
}
```

The constructor creates a new Counter with the value initialized to zero, wrapped in a Mutex for thread-safe access.

### Methods

**increment()**

```rust
fn increment(&self) {
    let mut num = self.value.lock().unwrap();
    *num += 1;
}
```

The `increment` method acquires the lock on the mutex, increments the value, and automatically releases the lock when `num` goes out of scope.

**get_value()**

```rust
fn get_value(&self) -> usize {
    *self.value.lock().unwrap()
}
```

The `get_value` method acquires the lock, reads the current value, and returns it. The lock is released when the method returns.

## Shared State Example

The complete example demonstrates how to safely share a Counter between multiple threads using Arc and Mutex.

### Code Structure

```rust
let counter = Arc::new(Counter::new());

for thread_id in 0..num_threads {
    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        for i in 0..increments_per_thread {
            counter_clone.increment();
        }
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

The pattern consists on:

`Arc::new()` -> Wraps the Counter in an Arc to allow shared ownership;

`Arc::clone()` -> Each thread receives its own Arc clone pointing to the same Counter;

`thread::spawn()` -> Each thread captures its Arc clone via the `move` closure;

`.join()` -> The main thread waits for all worker threads to complete.

## Why This Approach Works

- **Arc** ensures that the Counter won't be deallocated while threads are still using it;
- **Mutex** ensures that only one thread can modify the counter at a time, preventing race conditions;
- **Atomic Operations** ensure that incrementing and decrementing the reference count is thread-safe.

The combination of Arc and Mutex allows multiple threads to safely share and modify the same data without compilation errors or runtime data races.

## What Happens Without Synchronization

Without proper synchronization primitives like Mutex, multiple threads accessing and modifying the same data would result in race conditions. Each thread might read the value, increment it, and write it back without waiting for other threads to finish, leading to lost updates and incorrect final values.

With Mutex in place, each thread must acquire the lock before accessing the data, ensuring that all increments are correctly counted and no updates are lost.
