use std::time::Instant; 

fn main() {
    let n = 20_000_000usize; // 20_000_000usize= 20 million elements
    let data = vec![1u64; n]; // what does v! do here?: creates a vector of size n, filled with 1u64 values where 1u64 is an unsigned 64-bit integer

    // Sequential access
    let t1 = Instant::now();
    let mut sum1 = 0u64;
    for i in 0..n {
        sum1 = sum1.wrapping_add(data[i]);
    }
    let seq = t1.elapsed();

    // Pseudo-random access
    let t2 = Instant::now();
    let mut sum2 = 0u64;
    let mut idx = 0usize;
    for _ in 0..n {
        idx = (idx * 1664525 + 1013904223) % n; // Linear congruential generator: generates pseudo-random indices
        sum2 = sum2.wrapping_add(data[idx]);
    }
    let rnd = t2.elapsed();

    println!("sequential: sum={} elapsed_ms={}", sum1, seq.as_millis());
    println!("random:     sum={} elapsed_ms={}", sum2, rnd.as_millis());
}
