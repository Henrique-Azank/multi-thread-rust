//! Parallel iteration examples using Rayon
//! 
//! This module demonstrates data parallelism using the Rayon library,
//! which makes it easy to convert sequential computations into parallel ones.

use rayon::prelude::*;
use std::time::Instant;
use crate::common;

/// A simple CPU-intensive function for benchmarking
fn compute_intensive(n: u64) -> u64 {
    (0..n).fold(0, |acc, x| acc.wrapping_add(x * x))
}

/// Sequential map example
fn sequential_map(data: &[u64]) -> Vec<u64> {
    data.iter().map(|&x| compute_intensive(x)).collect()
}

/// Parallel map example using Rayon
fn parallel_map(data: &[u64]) -> Vec<u64> {
    data.par_iter().map(|&x| compute_intensive(x)).collect()
}

/// Sequential filter and sum
fn sequential_filter_sum(data: &[u64]) -> u64 {
    data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}

/// Parallel filter and sum
fn parallel_filter_sum(data: &[u64]) -> u64 {
    data.par_iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}

/// Parallel sorting
fn parallel_sort(data: &mut [u64]) {
    data.par_sort_unstable();
}

/// Run the parallel iteration examples
pub fn run(size: usize, benchmark: bool) {
    common::print_info(&format!("Collection size: {}", size));
    common::print_info(&format!("Number of CPUs: {}", num_cpus::get()));
    
    println!();
    
    if benchmark {
        run_benchmark(size);
    } else {
        run_examples(size);
    }
}

fn run_examples(size: usize) {
    // Create test data
    let data: Vec<u64> = (0..size as u64).collect();
    
    common::print_info("Example 1: Parallel Map");
    let start = Instant::now();
    let sample_size = size.min(1000);
    let result = parallel_map(&data[..sample_size]);
    let duration = start.elapsed();
    common::print_success(&format!(
        "Processed {} items in {:?}",
        sample_size, duration
    ));
    common::print_info(&format!("First 5 results: {:?}", &result[..5.min(sample_size)]));
    
    println!();
    
    common::print_info("Example 2: Parallel Filter and Sum");
    let start = Instant::now();
    let sum = parallel_filter_sum(&data);
    let duration = start.elapsed();
    common::print_success(&format!("Sum of squares of even numbers: {}", sum));
    common::print_info(&format!("Computed in {:?}", duration));
    
    println!();
    
    common::print_info("Example 3: Parallel Sort");
    let mut data_to_sort: Vec<u64> = (0..size as u64).rev().collect();
    let start = Instant::now();
    parallel_sort(&mut data_to_sort);
    let duration = start.elapsed();
    common::print_success(&format!("Sorted {} items in {:?}", size, duration));
    common::print_info(&format!("First 5 sorted: {:?}", &data_to_sort[..5.min(size)]));
    
    println!();
    
    common::print_info("Example 4: Parallel iteration with custom thread pool");
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    
    let sum = pool.install(|| {
        data.par_iter()
            .filter(|&&x| x % 3 == 0)
            .sum::<u64>()
    });
    
    common::print_success(&format!("Sum of numbers divisible by 3: {}", sum));
}

fn run_benchmark(size: usize) {
    common::print_header("Benchmark Mode: Sequential vs Parallel");
    
    // Create test data
    let data: Vec<u64> = (0..size as u64).map(|x| x % 1000).collect();
    
    // Benchmark 1: Map
    println!();
    common::print_info("Benchmark 1: Map operation");
    
    let start = Instant::now();
    let seq_result = sequential_map(&data[..size.min(10000)]);
    let seq_duration = start.elapsed();
    common::print_info(&format!("Sequential: {:?}", seq_duration));
    
    let start = Instant::now();
    let par_result = parallel_map(&data[..size.min(10000)]);
    let par_duration = start.elapsed();
    common::print_info(&format!("Parallel:   {:?}", par_duration));
    
    let speedup = seq_duration.as_secs_f64() / par_duration.as_secs_f64();
    common::print_success(&format!("Speedup: {:.2}x", speedup));
    
    // Verify results match
    assert_eq!(seq_result, par_result);
    
    // Benchmark 2: Filter and Sum
    println!();
    common::print_info("Benchmark 2: Filter and Sum operation");
    
    let start = Instant::now();
    let seq_sum = sequential_filter_sum(&data);
    let seq_duration = start.elapsed();
    common::print_info(&format!("Sequential: {:?}", seq_duration));
    
    let start = Instant::now();
    let par_sum = parallel_filter_sum(&data);
    let par_duration = start.elapsed();
    common::print_info(&format!("Parallel:   {:?}", par_duration));
    
    let speedup = seq_duration.as_secs_f64() / par_duration.as_secs_f64();
    common::print_success(&format!("Speedup: {:.2}x", speedup));
    
    // Verify results match
    assert_eq!(seq_sum, par_sum);
    
    // Benchmark 3: Sort
    println!();
    common::print_info("Benchmark 3: Sorting");
    
    let mut seq_data = data.clone();
    let start = Instant::now();
    seq_data.sort_unstable();
    let seq_duration = start.elapsed();
    common::print_info(&format!("Sequential: {:?}", seq_duration));
    
    let mut par_data = data.clone();
    let start = Instant::now();
    parallel_sort(&mut par_data);
    let par_duration = start.elapsed();
    common::print_info(&format!("Parallel:   {:?}", par_duration));
    
    let speedup = seq_duration.as_secs_f64() / par_duration.as_secs_f64();
    common::print_success(&format!("Speedup: {:.2}x", speedup));
    
    // Verify results match
    assert_eq!(seq_data, par_data);
    
    println!();
    common::print_success("All benchmarks completed! Results verified.");
}
