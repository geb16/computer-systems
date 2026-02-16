use std::time::Instant;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::hint::black_box;

fn main() {
    let size: usize = 50_000_000;

    let data: Vec<u64> = (0..size).map(|n| n as u64).collect();

    // Sequential
    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..size {
        sum = sum.wrapping_add(data[i]);
    }
    black_box(sum); // prevent optimization
    let seq_time = start.elapsed();
    println!("Sequential time: {:?}", seq_time);

    // Random
    let mut indices: Vec<usize> = (0..size).collect();
    indices.shuffle(&mut thread_rng());

    let start = Instant::now();
    let mut sum2 = 0u64;
    for i in 0..size {
        sum2 = sum2.wrapping_add(data[indices[i]]);
    }
    black_box(sum2); // black_box is 
    let rand_time = start.elapsed();
    println!("Random time: {:?}", rand_time);
}
