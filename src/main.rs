
// Project dependencies
use multi_thread_rust::{common::print_header, Cli, Commands, tools::*};
use clap::Parser;

fn main() {

    // Instantiate the CLI parser and match on the provided command
    let cli = Cli::parse();
    
    // Match the subcommand ENUM
    match cli.command {
        Commands::ThreadPool { threads, num_tasks } => {
            print_header("Thread Pool Example");
            thread_pool::run(threads, num_tasks);
        }
        Commands::MessagePassing { senders, messages } => {
            print_header("Message Passing Example");
            message_passing::run(senders, messages);
        }
        Commands::SharedState { threads, increments } => {
            print_header("Shared State Example");
            shared_state::run(threads, increments);
        }
        Commands::AsyncTasks { tasks, delay } => {
            print_header("Async Tasks Example");
            async_tasks::run(tasks, delay);
        }
        Commands::ParallelIteration { size, benchmark } => {
            print_header("Parallel Iteration Example");
            parallel_iteration::run(size, benchmark);
        }
    }
}
