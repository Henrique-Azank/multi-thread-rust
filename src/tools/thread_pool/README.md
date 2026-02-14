
# Thread Pool 

The goal is to create a fixed number of threads (Workers) that all listen to a single source of work (Receiver). The main thread sends tasks (Jobs) into a channel, and whichever worker is free grabs the next task.

## Job type

The Job type defines a task that should be processed in this system. 

### Code Structure

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
```

The implementation of the `Job` type consists on:

`Box` -> Since we do not know the size of the task closure (function) will be at compile time, we must store it in the heap;

`dyn FnOnce()` -> The function / tasks that should be called in the task and can only be called once;

`Send` -> Indicates that the closure can be safely moved from the main thread to the worker thread; 

`'static` -> The closure may not borrow data from the main thread stack (that might disappear). The closure must own its data. 

## Worker Struct

This is the actual work unit used in thread pool. It is a wrapper around an OS thread, instantiated during the script setup and kept alive to receive the `Job` instances that require processing

### Code Structure

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

The implementation of `Worker` consists on:

`id` -> Ordinary `usize` based identification for the worker

`thread` -> It holds the `thread::JoinHandle<()>` so we can wait for the thread to finish later (The thread handle is not the thread itself). The generic parameter inside of it represents the return value of the thread, in this case the Unit type `()`, since we do not return anything.

The thread handle posesses a very powerful method called `.join()`. The method pauses the main thread and waits for the execution of the thread whose handle we called `.join()`. If we did not wait for the thread join, the main thread would finish executing before we were sure that the worker thread finished all of its jobs. 

### Instantiation 

The usage of the `Worker` struct can be more easily understood by its `new` method, that describes the creation process of a new instance.

```rust
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
```

The new method receives as a parameter a thread-safe receiver reference (for receiving messaes from the thread pool sender)
and an id for keeping track of the worker. The thread´s closure creates a loop that waits for the next message
(job) from the thread poll.

## Thread Pool Struct

Consists on the main manager of this concurrency structure. 

### Code Structure 

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
```

The implementation of `ThreadPool` struct consists on:

`workers` -> A `Vec` / list of currently active `Worker` threads (whose type we defined previosly). These are the threads we can send the jobs to. 

`sender` -> The "sending" end of a `mpsc` created channel with the `Job` type defined as the sent data. We wrap it as an `Option` so we can later take ownership of it during shutdown (important for the `Drop` trait implementation)

### Instantiation

The shared state management of this structure can be furthern understood by inspecting its constructor method

```rust
/// Create a new ThreadPool with the specified number of threads
pub fn new(size: usize) -> ThreadPool {

    // The number of threads must be greater than zero
    assert!(size > 0);

    // Create a channel for sending jobs to the worker threads
    let (sender, receiver) = mpsc::channel();
    
    // Wrap the receiver in an Arc and Mutex to allow shared ownership and thread-safe access
    let receiver = Arc::new(Mutex::new(receiver));
    
    // Instantiate a new vector to hold the worker threads
    let mut workers = Vec::with_capacity(size);

    // Create the specified number of worker threads and add them to the pool
    for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    // Create the ThreadPool instance with the workers and sender
    ThreadPool {
        workers,
        sender: Some(sender),
    }
}
```

Here we can see that the thread pool is created and the desired number of workers
is instantiated, each receiving an Atomically Referenced Counter `Arc` of the receiver 
struct from the created thread pool channel. That way, the manager can send tasks to the 
created workers without actively blocking the main thread and without need to specify the 
worker ID. 

### Usage 

The actual usage of the thread pool is pretty straight forward. The job that needs to be performed
is sent to the channel via the `sender` as a message. Whichever thread currently retains the lock for
the `receiver` performs the tasks and frees the next worker to take the lock and await for the next 
message / job. 

```rust
pub fn execute<F>(&self, f: F)
where
    F: FnOnce() + Send + 'static,
{
    let job = Box::new(f);
    self.sender.as_ref().unwrap().send(job).unwrap();
}
```