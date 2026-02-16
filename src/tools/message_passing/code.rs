//! Message passing examples using channels
//! 
//! This module demonstrates communication between threads using
//! message passing with channels (mpsc and crossbeam).

/*
    NOTE: MPSC means Multiple Producesr, Single Consumer
*/

// Base dependencies
use std::sync::mpsc;
use std::{thread, thread::JoinHandle};
use std::time::Duration;

// Third-party dependencies
use crossbeam::channel;

// Project dependencies
use crate::common;

/// Example using standard library mpsc channels
fn run_mpsc(num_senders: usize, messages_per_sender: usize) {

    // Instantiate a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // Vector to hold the sender thread handles
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // For the specified number of sender threads, spawn a new thread that sends messages to the receiver
    for sender_id in 0..num_senders {

        // Clone the transmitter for each sender thread to allow multiple producers
        let tx_clone = tx.clone();

        // Spawn a sender thread that sends a series of messages to the receiver
        let handle = thread::spawn(move || {
            for msg_num in 0..messages_per_sender {
                let message = format!("Message {} from sender {}", msg_num, sender_id);
                tx_clone.send(message).unwrap();
                common::print_info(&format!("Sender {} sent message {}", sender_id, msg_num));
                thread::sleep(Duration::from_millis(50));
            }
        });

        // Append the sender thread handle to the vector for later joining
        handles.push(handle);
    }

    // Drop the original sender so the receiver knows when all senders are done
    drop(tx);

    // Spawn receiver thread
    let receiver_handle = thread::spawn(move || {
        let mut count = 0;
        for received in rx {
            println!("📨 Received: {}", received);
            count += 1;
        }
        common::print_success(&format!("Receiver got {} total messages", count));
    });

    // Wait for all senders to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Wait for receiver to complete
    receiver_handle.join().unwrap();
}

/// Example using crossbeam channels (supports multiple consumers)
fn run_crossbeam(num_senders: usize, messages_per_sender: usize) {
    let (tx, rx) = channel::unbounded();
    let mut handles = vec![];

    // Multiple senders
    for sender_id in 0..num_senders {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for msg_num in 0..messages_per_sender {
                let message = format!("Crossbeam message {} from sender {}", msg_num, sender_id);
                tx_clone.send(message).unwrap();
                thread::sleep(Duration::from_millis(30));
            }
        });
        handles.push(handle);
    }

    // Drop the original sender
    drop(tx);

    // Multiple receivers
    let num_receivers = 2;
    let mut receiver_handles = vec![];
    
    for receiver_id in 0..num_receivers {
        let rx_clone = rx.clone();
        let handle = thread::spawn(move || {
            let mut count = 0;
            while let Ok(message) = rx_clone.recv() {
                println!("📬 Receiver {} got: {}", receiver_id, message);
                count += 1;
            }
            common::print_success(&format!("Receiver {} processed {} messages", receiver_id, count));
        });
        receiver_handles.push(handle);
    }

    // Drop original receiver
    drop(rx);

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    for handle in receiver_handles {
        handle.join().unwrap();
    }
}

/// Run the message passing example with standard library channels
pub fn run(num_senders: usize, messages_per_sender: usize) {
    common::print_info("Running standard library mpsc channel example");
    run_mpsc(num_senders, messages_per_sender);
    
    println!();
    
    common::print_info("Running crossbeam channel example");
    run_crossbeam(num_senders, messages_per_sender);
}

