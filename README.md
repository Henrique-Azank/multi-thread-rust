# Multi-threaded Rust Examples

A comprehensive CLI tool demonstrating various parallelism and concurrency patterns in Rust.

## Features

This project showcases different concurrency and parallelism approaches:

- **Shared State**: Safe shared state using Arc and Mutex
- **Thread Pool**: Custom thread pool implementation for task execution
- **Message Passing**: Channel-based communication between threads (mpsc and crossbeam)

- **Async/Await**: Asynchronous programming with Tokio runtime
- **Parallel Iteration**: Data parallelism with Rayon

## Prerequisites

- Rust 1.70+ (with cargo)
- macOS, Linux, or Windows

## Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd multi-thread-rust

# Build the project
cargo build --release
```

## Usage

The CLI provides subcommands for each example type:

### Thread Pool

Create a thread pool and execute tasks:

```bash
# Use 4 threads to execute 10 tasks
cargo run --release -- thread-pool --threads 4 --num-tasks 10

# Short form
cargo run --release -- thread-pool -t 4 -n 10
```

### Message Passing

Demonstrate channel-based communication:

```bash
# Use 3 sender threads, each sending 5 messages
cargo run --release -- message-passing --senders 3 --messages 5

# Short form
cargo run --release -- message-passing -s 3 -m 5
```

### Shared State

Show safe shared state with Mutex and Arc:

```bash
# Use 5 threads, each incrementing 1000 times
cargo run --release -- shared-state --threads 5 --increments 1000

# Short form
cargo run --release -- shared-state -t 5 -i 1000
```

### Async Tasks

Demonstrate async/await with Tokio:

```bash
# Run 5 concurrent async tasks with 100ms delay
cargo run --release -- async-tasks --tasks 5 --delay 100

# Short form
cargo run --release -- async-tasks -t 5 -d 100
```

### Parallel Iteration

Show data parallelism with Rayon:

```bash
# Process a collection of 1,000,000 items
cargo run --release -- parallel-iteration --size 1000000

# Run in benchmark mode to compare sequential vs parallel
cargo run --release -- parallel-iteration --size 1000000 --benchmark

# Short form
cargo run --release -- parallel-iteration -s 1000000 -b
```

## Project Structure

```
multi-thread-rust/
├── Cargo.toml              # Project manifest and dependencies
├── src/
│   ├── main.rs             # CLI entry point with clap
│   ├── lib.rs              # Library root with CLI definitions
│   ├── common.rs           # Common utilities (print_header)
│   └── tools/              # Concurrency and parallelism examples
│       ├── mod.rs          # Tools module root
│       ├── thread_pool/    # Thread pool implementation
│       │   ├── mod.rs
│       │   └── code.rs
│       ├── message_passing/ # Channel-based communication
│       │   ├── mod.rs
│       │   └── code.rs
│       ├── shared_state/   # Arc/Mutex examples
│       │   ├── mod.rs
│       │   └── code.rs
│       ├── async_tasks/    # Tokio async/await examples
│       │   ├── mod.rs
│       │   └── code.rs
│       └── parallel_iteration/ # Rayon parallel processing
│           ├── mod.rs
│           └── code.rs
└── README.md
```

## Dependencies

- **clap**: Command-line argument parsing
- **tokio**: Async runtime
- **rayon**: Data parallelism library
- **crossbeam**: Advanced concurrency utilities
- **colored**: Terminal output coloring
- **num_cpus**: CPU core detection

## Examples Explained

### Thread Pool
Demonstrates a custom thread pool implementation that:
- Creates a fixed number of worker threads
- Distributes tasks across workers using channels
- Cleanly shuts down when dropped

### Message Passing
Shows two channel implementations:
- Standard library `mpsc` (multiple producer, single consumer)
- Crossbeam channels (multiple producer, multiple consumer)

### Shared State
Illustrates safe concurrent access to shared data:
- Uses `Arc` for shared ownership across threads
- Uses `Mutex` to ensure exclusive access during modifications
- Prevents data races at compile time

### Async Tasks
Explores asynchronous programming:
- Concurrent task execution with `tokio::spawn`
- The `join!` macro for parallel async operations
- Sequential vs concurrent execution comparison
- Timeout handling

### Parallel Iteration
Demonstrates Rayon's data parallelism:
- Parallel map operations
- Parallel filtering and reduction
- Parallel sorting
- Performance benchmarking mode

## Learning Resources

- [The Rust Programming Language - Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rayon Documentation](https://docs.rs/rayon/)
- [Crossbeam Documentation](https://docs.rs/crossbeam/)

## License

MIT
