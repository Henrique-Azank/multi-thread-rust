# Async Tasks

The goal is to demonstrate asynchronous programming in Rust using the Tokio runtime and the async/await syntax. This module compares concurrent execution, sequential execution, and timeout handling.

## Tokio Runtime

Tokio provides an asynchronous runtime that schedules and runs async tasks.

### Code Structure

```rust
let rt = tokio::runtime::Runtime::new().unwrap();

rt.block_on(async {
    // async examples
});
```

The implementation consists on:

`Runtime::new()` -> Creates a multi-threaded Tokio runtime with default settings;

`block_on()` -> Runs an async block to completion on the runtime.

## Async Task Function

The core async operation is represented by a task that sleeps and returns a message.

### Code Structure

```rust
async fn async_task(id: usize, delay_ms: u64) -> String {
    common::print_info(&format!("Task {} started", id));
    sleep(Duration::from_millis(delay_ms)).await;
    let result = format!("Task {} completed after {}ms", id, delay_ms);
    common::print_success(&result);
    result
}
```

The task:

- Logs its start
- Awaits a timer using `tokio::time::sleep`
- Returns a completion message

## Concurrent Execution

Multiple tasks can run concurrently by spawning them and awaiting their handles.

### Code Structure

```rust
let mut handles = vec![];
for i in 0..num_tasks {
    let handle = task::spawn(async_task(i, delay_ms));
    handles.push(handle);
}

for handle in handles {
    results.push(handle.await.unwrap());
}
```

The pattern consists on:

`task::spawn()` -> Schedules each async task on the runtime;

`handle.await` -> Waits for each task to finish and retrieves its result.

## join! Macro

The `tokio::join!` macro runs multiple async expressions concurrently and returns all results.

### Code Structure

```rust
let (r1, r2, r3) = tokio::join!(
    async_task(100, delay_ms),
    async_task(101, delay_ms),
    async_task(102, delay_ms),
);
```

This is a convenient way to run a fixed set of tasks in parallel without explicitly spawning them.

## Sequential Execution

Async tasks can also be executed sequentially by awaiting each call in order.

### Code Structure

```rust
for i in 0..num_tasks {
    async_task(i + 200, delay_ms).await;
}
```

This is useful for comparison, as the total runtime is roughly the sum of all individual delays.

## Timeout Handling

Tokio provides a timeout utility to cancel or fail tasks that take too long.

### Code Structure

```rust
let timeout_duration = Duration::from_millis(delay_ms / 2);
let task_future = async_task(300, delay_ms);

match tokio::time::timeout(timeout_duration, task_future).await {
    Ok(result) => common::print_success(&format!("Task completed: {}", result)),
    Err(_) => common::print_warning(&format!(
        "Task timed out after {:?}",
        timeout_duration
    )),
}
```

If the task does not complete before the timeout, an error is returned and the timeout path is executed.

## Why Async Works Well Here

- The runtime can schedule many tasks without dedicating a thread per task.
- Await points allow the executor to switch to other tasks while waiting.
- The code stays close to synchronous style while remaining non-blocking.

This module demonstrates practical patterns for asynchronous execution and timing in Rust.
