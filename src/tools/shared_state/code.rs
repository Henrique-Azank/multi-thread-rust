//! Shared state examples using Arc and Mutex
//! 
//! This module demonstrates how to safely share state between threads
//! using Arc (Atomic Reference Counting) and Mutex (Mutual Exclusion).

use std::sync::{Arc, Mutex};
use std::thread;
use crate::common;

/// A simple counter protected by a Mutex
struct Counter {
    value: Mutex<usize>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut num = self.value.lock().unwrap();
        *num += 1;
    }

    fn get_value(&self) -> usize {
        *self.value.lock().unwrap()
    }
}

/// Run the shared state example
pub fn run(num_threads: usize, increments_per_thread: usize) {
    common::print_info(&format!(
        "Creating {} threads, each incrementing a counter {} times",
        num_threads, increments_per_thread
    ));

    let counter = Arc::new(Counter::new());
    let mut handles = vec![];

    let start = std::time::Instant::now();

    for thread_id in 0..num_threads {
        let counter_clone = Arc::clone(&counter);
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
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    let final_value = counter.get_value();
    let expected_value = num_threads * increments_per_thread;

    println!();
    common::print_success(&format!("Final counter value: {}", final_value));
    common::print_success(&format!("Expected value: {}", expected_value));
    
    if final_value == expected_value {
        common::print_success("✅ Counter is correct! No race conditions detected.");
    } else {
        common::print_warning("⚠️  Counter mismatch! This should not happen with Mutex.");
    }
    
    common::print_info(&format!("Total time: {:?}", duration));
    common::print_info(&format!(
        "Average time per increment: {:?}",
        duration / (num_threads * increments_per_thread) as u32
    ));
}

/// Demonstrate the danger of shared mutable state without synchronization
#[allow(dead_code)]
pub fn run_unsafe_example(num_threads: usize, increments_per_thread: usize) {
    common::print_warning("WARNING: This example shows INCORRECT concurrent code");
    common::print_warning("This is for educational purposes only - DO NOT use in production");
    
    // Note: This would require unsafe code to actually compile
    // We're keeping this as documentation of what NOT to do
    common::print_info(&format!(
        "Without proper synchronization (Mutex), {} threads incrementing {} times each",
        num_threads, increments_per_thread
    ));
    common::print_info("would likely result in a final value less than the expected value");
    common::print_info(&format!("Expected: {}", num_threads * increments_per_thread));
}
