//! Shared state examples using Arc and Mutex
//! 
//! This module demonstrates how to safely share state between threads
//! using Arc (Atomic Reference Counting) and Mutex (Mutual Exclusion).

// Base dependencies
use std::sync::{Arc, Mutex};
use std::thread;

// Project dependencies 
use crate::common;

/// A simple counter protected by a Mutex
struct Counter {
    value: Mutex<usize>,
}

// Base method implementations for Counter
impl Counter {
    // Structure constructor
    fn new() -> Self {
        Counter {
            value: Mutex::new(0),
        }
    }

    // Increment the counter safely using the Mutex
    fn increment(&self) {

        // Get the safe lock on the counter value
        let mut num = self.value.lock().unwrap();

        // Increment the counter
        *num += 1;
    }

    // Get the current value of the counter
    fn get_value(&self) -> usize {
        *self.value.lock().unwrap()
    }
}

/// Run the shared state example
pub fn run(num_threads: usize, increments_per_thread: usize) {

    // Log the parameters of the test
    common::print_info(&format!(
        "Creating {} threads, each incrementing a counter {} times",
        num_threads, increments_per_thread
    ));

    // Create a shared counter wrapped in an Arc to allow multiple ownership across threads
    let counter = Arc::new(Counter::new());

    // Vector to hold the thread handles so we can wait for them to finish
    let mut handles = vec![];

    // Create a timer to measure how long the increments take
    let start = std::time::Instant::now();

    // Spawn multiple threads to increment the counter concurrently
    for thread_id in 0..num_threads {

        // Create a new reference to the shared counter for each thread
        let counter_clone = Arc::clone(&counter);

        // Spawn a thread that will increment the counter a specified number of times
        let handle = thread::spawn(move || {
            for i in 0..increments_per_thread {
                counter_clone.increment();
                if i % (increments_per_thread / 10).max(1) == 0 {
                    common::print_info(&format!(
                        "Thread {} progress: {:.0}%",
                        thread_id,
                        (i as f64 / increments_per_thread as f64) * 100.0
                    ));
                }
            }
            common::print_success(&format!("Thread {} completed all increments", thread_id));
        });

        // Push the thread handle to the vector so we can join later
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Calculate the total duration of the increments
    let duration = start.elapsed();
    let final_value = counter.get_value();
    let expected_value = num_threads * increments_per_thread;

    // Log the final results and check for correctness
    common::print_success(&format!("Final counter value: {}", final_value));
    common::print_success(&format!("Expected value: {}", expected_value));
    
    if final_value == expected_value {
        common::print_success("✅ Counter is correct! No race conditions detected.");
    } else {
        common::print_warning("⚠️  Counter mismatch! This should not happen with Mutex.");
    }

    // Log the total time taken for the increments
    common::print_info(&format!("Total time: {:?}", duration));

}
