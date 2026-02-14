//! Multi-threaded and concurrent programming examples in Rust
//! 
//! This library provides various examples and implementations of parallelism
//! and concurrency patterns in Rust.


// Third-party dependencies
use clap::{Parser, Subcommand};

// Re-exporting modules for easier access from main.rs
pub mod tools;
pub mod common;

// Base CLI definitions for the application
#[derive(Parser)]
#[command(name = "multi-thread-rust")]
#[command(about = "A CLI tool to demonstrate parallelism and concurrency in Rust", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// Create an enum for the different command options
#[derive(Subcommand)]
pub enum Commands {
    /// Run thread pool examples
    ThreadPool {
        /// Number of threads in the pool
        #[arg(short, long, default_value_t = 4)]
        threads: usize,
        
        /// Number of tasks to execute
        #[arg(short = 'n', long, default_value_t = 10)]
        num_tasks: usize,
    },
    
    /// Run message passing examples using channels
    MessagePassing {
        /// Number of sender threads
        #[arg(short, long, default_value_t = 3)]
        senders: usize,
        
        /// Number of messages per sender
        #[arg(short, long, default_value_t = 5)]
        messages: usize,
    },
    
    /// Run shared state examples using Mutex and Arc
    SharedState {
        /// Number of threads to spawn
        #[arg(short, long, default_value_t = 5)]
        threads: usize,
        
        /// Number of increments per thread
        #[arg(short, long, default_value_t = 1000)]
        increments: usize,
    },
    
    /// Run async/await examples with Tokio
    AsyncTasks {
        /// Number of concurrent tasks
        #[arg(short, long, default_value_t = 5)]
        tasks: usize,
        
        /// Delay in milliseconds for each task
        #[arg(short, long, default_value_t = 100)]
        delay: u64,
    },
    
    /// Run parallel iteration examples with Rayon
    ParallelIteration {
        /// Size of the collection to process
        #[arg(short, long, default_value_t = 1000000)]
        size: usize,
        
        /// Enable benchmark mode
        #[arg(short, long)]
        benchmark: bool,
    },
}

