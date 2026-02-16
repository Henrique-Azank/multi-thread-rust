# Message Passing

The goal is to communicate between threads by sending messages through channels instead of sharing mutable state. This module demonstrates two approaches:

- Standard library `mpsc` (multiple producers, single consumer)
- `crossbeam` channels (multiple producers, multiple consumers)

Message passing makes ownership explicit and avoids data races by moving data between threads.

## Standard Library `mpsc`

The `mpsc` channel from the standard library provides multiple senders and a single receiver.

### Code Structure

```rust
let (tx, rx) = mpsc::channel();
let tx_clone = tx.clone();
```

The implementation consists on:

`mpsc::channel()` -> Creates a sender (`tx`) and receiver (`rx`) pair;

`tx.clone()` -> Creates an additional sender so multiple producer threads can send messages;

`drop(tx)` -> Dropping the original sender lets the receiver know when all senders are finished.

### Sender Threads

```rust
let handle = thread::spawn(move || {
    for msg_num in 0..messages_per_sender {
        let message = format!("Message {} from sender {}", msg_num, sender_id);
        tx_clone.send(message).unwrap();
        thread::sleep(Duration::from_millis(50));
    }
});
```

Each sender thread owns a cloned transmitter and sends a sequence of messages to the receiver.

### Receiver Thread

```rust
let receiver_handle = thread::spawn(move || {
    let mut count = 0;
    for received in rx {
        println!("📨 Received: {}", received);
        count += 1;
    }
    common::print_success(&format!("Receiver got {} total messages", count));
});
```

The receiver iterates over `rx` until all senders are dropped. This loop ends automatically when the channel closes.

## Crossbeam Channels

Crossbeam channels support multiple producers and multiple consumers. They are often faster and more flexible than the standard library channel.

### Code Structure

```rust
let (tx, rx) = channel::unbounded();
let tx_clone = tx.clone();
let rx_clone = rx.clone();
```

The implementation consists on:

`channel::unbounded()` -> Creates a channel with no capacity limit;

`tx.clone()` -> Enables multiple sender threads;

`rx.clone()` -> Enables multiple receiver threads.

### Multiple Receivers

```rust
let handle = thread::spawn(move || {
    let mut count = 0;
    while let Ok(message) = rx_clone.recv() {
        println!("📬 Receiver {} got: {}", receiver_id, message);
        count += 1;
    }
    common::print_success(&format!("Receiver {} processed {} messages", receiver_id, count));
});
```

Each receiver thread processes messages until the channel is closed.

### Closing the Channel

```rust
drop(tx);
drop(rx);
```

Dropping all senders closes the channel and lets receivers exit their loops. Dropping the original receiver is required when multiple receiver clones exist.

## Why Message Passing Works

- Ownership is moved across threads, so there is no shared mutable state.
- Channels synchronize access implicitly, reducing the need for locks.
- The receiver loop ends cleanly when all senders are done.

This pattern is especially useful when tasks are naturally expressed as events or discrete units of work.
