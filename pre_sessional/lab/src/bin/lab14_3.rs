use std::thread;
use std::sync::mpsc;
use rand::Rng;
use std::time::Instant;

fn split_vector(numbers: &Vec<u32>, num_parts: usize) -> Vec<Vec<u32>> {
    let len = numbers.len();
    let part_size = len / num_parts;
    let mut parts = Vec::with_capacity(num_parts);
    
    for i in 0..num_parts {
        let start = i * part_size;
        let end = if i == num_parts - 1 { len } else { (i + 1) * part_size };
        parts.push(numbers[start..end].to_vec());
    }
    
    parts
}

fn main() {
    println!("Generating 1,000,000 random numbers...");
    let numbers: Vec<u32> = (0..1_000_000)
        .map(|_| rand::thread_rng().gen_range(1..=100))
        .collect();
    println!("Generation complete.\n");

    // Multi-threaded version
    println!("Running multi-threaded calculation...");
    let start_multi = Instant::now();
    let parts = split_vector(&numbers, 10);
    let (tx, rx) = mpsc::channel();
    
    let handles: Vec<_> = parts.into_iter()
        .map(|part| {
            let tx = tx.clone();
            thread::spawn(move || {
                let sum_of_squares: u64 = part.iter().map(|&x| x as u64 * x as u64).sum();
                tx.send(sum_of_squares).unwrap();
            })
        })
        .collect();
    
    drop(tx);
    let multi_threaded_sum: u64 = rx.iter().sum();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration_multi = start_multi.elapsed();
    println!("Multi-threaded sum of squares: {}", multi_threaded_sum);
    println!("Multi-threaded time taken: {:?}\n", duration_multi);

    // Single-threaded version
    println!("Running single-threaded calculation...");
    let start_single = Instant::now();
    let single_threaded_sum: u64 = numbers.iter().map(|&x| x as u64 * x as u64).sum();
    let duration_single = start_single.elapsed();
    
    println!("Single-threaded sum of squares: {}", single_threaded_sum);
    println!("Single-threaded time taken: {:?}\n", duration_single);

    // Performance comparison
    println!("Performance Comparison:");
    println!("----------------------");
    println!("Multi-threaded time: {:?}", duration_multi);
    println!("Single-threaded time: {:?}", duration_single);
    
    let speedup = duration_single.as_secs_f64() / duration_multi.as_secs_f64();
    println!("Speedup: {:.2}x", speedup);
    
    if speedup > 1.0 {
        println!("Multi-threaded version was faster.");
    } else {
        println!("Single-threaded version was faster.");
    }
}
