//! Thread pool implementation and examples
//! 
//! This module demonstrates how to create and use a custom thread pool
//! for executing tasks concurrently.

// Base dependencies
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Project dependencies
use crate::common;

/// Example of a job type that can be sent to the thread pool
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A simple thread pool implementation
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

/// Method implementations for ThreadPool
impl ThreadPool {

    /// Create a new ThreadPool with the specified number of threads
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Execute a job on the thread pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

}

// Gracefully shut down the thread pool when it goes out of scope
impl Drop for ThreadPool {

    // Base Drop implementation to clean up resources
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }

}

/// Worker struct representing a single thread in the pool
struct Worker {
    #[allow(dead_code)]
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    common::print_info(&format!("Worker {id} executing task"));
                    job();
                }
                Err(_) => {
                    common::print_info(&format!("Worker {id} shutting down"));
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// Run the thread pool example
pub fn run(num_threads: usize, num_tasks: usize) {
    common::print_info(&format!("Creating thread pool with {} threads", num_threads));
    let pool = ThreadPool::new(num_threads);

    common::print_info(&format!("Submitting {} tasks", num_tasks));
    
    for i in 0..num_tasks {
        pool.execute(move || {
            let thread_id = thread::current().id();
            println!("Task {} executing on thread {:?}", i, thread_id);
            // Simulate some work
            thread::sleep(std::time::Duration::from_millis(100));
        });
    }

    common::print_success("All tasks submitted");
    common::print_info("Waiting for all tasks to complete...");
    
    // Pool will be dropped here, waiting for all tasks to complete
    drop(pool);
    
    common::print_success("All tasks completed!");
}
