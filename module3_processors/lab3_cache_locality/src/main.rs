use std::time::Instant;

fn main() {
    let n = 50_000_000usize;

    // Vec<u64> is contiguous in memory.
    let data = vec![1u64; n];

    // Sequential access: cache-friendly
    let t1 = Instant::now();
    let mut s1 = 0u64;
    for i in 0..n {
        s1 = s1.wrapping_add(data[i]);
    }
    let e1 = t1.elapsed();

    // Strided access: less cache-friendly (skips around)
    // stride chosen to jump across cache lines frequently
    let stride = 64usize;
    let t2 = Instant::now();
    let mut s2 = 0u64;
    let mut i = 0usize;
    while i < n {
        s2 = s2.wrapping_add(data[i]);
        i += stride;
    }
    let e2 = t2.elapsed();

    println!("sequential sum={s1} elapsed_ms={}", e1.as_millis());
    println!("strided    sum={s2} elapsed_ms={}", e2.as_millis());
}
