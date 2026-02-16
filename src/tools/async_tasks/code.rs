//! Async/await examples using Tokio
//! 
//! This module demonstrates asynchronous programming in Rust using
//! the Tokio runtime and async/await syntax.

// Third-party dependencies
use tokio::time::{sleep, Duration, Instant};
use tokio::task;

// Project dependencies
use crate::common;

/// Simulate an async task that takes some time to complete
async fn async_task(id: usize, delay_ms: u64) -> String {

    // Log the task start
    common::print_info(&format!("Task {} started", id));

    // Simulate its execution
    sleep(Duration::from_millis(delay_ms)).await;

    // Print the result 
    let result = format!("Task {} completed after {}ms", id, delay_ms);
    common::print_success(&result);

    // Return the result
    result
}

/// Example of spawning multiple concurrent async tasks
async fn spawn_concurrent_tasks(num_tasks: usize, delay_ms: u64) {
    common::print_info(&format!("Spawning {} concurrent async tasks", num_tasks));
    let start = Instant::now();

    let mut handles = vec![];
    
    for i in 0..num_tasks {
        let handle = task::spawn(async_task(i, delay_ms));
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    
    let duration = start.elapsed();
    
    println!();
    common::print_success(&format!("All {} tasks completed", num_tasks));
    common::print_info(&format!("Total time: {:?}", duration));
    common::print_info(&format!(
        "Tasks ran concurrently - total time (~{}ms) is much less than sequential time ({}ms)",
        duration.as_millis(),
        num_tasks as u64 * delay_ms
    ));
}

/// Example of using join! macro for concurrent execution
async fn join_macro_example(delay_ms: u64) {
    common::print_info("Running join! macro example");
    let start = Instant::now();
    
    let (r1, r2, r3) = tokio::join!(
        async_task(100, delay_ms),
        async_task(101, delay_ms),
        async_task(102, delay_ms),
    );
    
    let duration = start.elapsed();
    
    println!();
    common::print_success("join! completed");
    common::print_info(&format!("Results: {}, {}, {}", r1, r2, r3));
    common::print_info(&format!("Time: {:?}", duration));
}

/// Example of sequential async/await
async fn sequential_example(num_tasks: usize, delay_ms: u64) {
    common::print_info(&format!("Running {} async tasks sequentially", num_tasks));
    let start = Instant::now();
    
    for i in 0..num_tasks {
        async_task(i + 200, delay_ms).await;
    }
    
    let duration = start.elapsed();
    
    println!();
    common::print_success("Sequential execution completed");
    common::print_info(&format!("Total time: {:?}", duration));
    common::print_info(&format!(
        "Sequential time (~{}ms) ≈ sum of all individual tasks",
        duration.as_millis()
    ));
}

/// Example of async task with timeout
async fn timeout_example(delay_ms: u64) {
    common::print_info("Running timeout example");
    
    let timeout_duration = Duration::from_millis(delay_ms / 2);
    let task_future = async_task(300, delay_ms);
    
    match tokio::time::timeout(timeout_duration, task_future).await {
        Ok(result) => common::print_success(&format!("Task completed: {}", result)),
        Err(_) => common::print_warning(&format!(
            "Task timed out after {:?}",
            timeout_duration
        )),
    }
}

/// Run all async examples
pub fn run(num_tasks: usize, delay_ms: u64) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        // Concurrent execution
        spawn_concurrent_tasks(num_tasks, delay_ms).await;
        
        println!("\n{}", "=".repeat(60));
        
        // join! macro
        join_macro_example(delay_ms).await;
        
        println!("\n{}", "=".repeat(60));
        
        // Sequential execution for comparison
        sequential_example(3, delay_ms).await;
        
        println!("\n{}", "=".repeat(60));
        
        // Timeout example
        timeout_example(delay_ms).await;
    });
}
