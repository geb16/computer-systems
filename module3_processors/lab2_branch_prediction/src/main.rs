use std::time::Instant;

fn predictable_branch(n: u64) -> u64 {
    let mut acc = 0u64; // 
    for i in 0..n {
        // Highly predictable: true for most of the loop after warm-up.
        if i < n - 1 {
            acc = acc.wrapping_add(1);
        }
    }
    acc
}

fn unpredictable_branch(n: u64) -> u64 {
    let mut acc = 0u64;
    // Simple pseudo-random pattern (no RNG crate needed).
    // This creates less predictable branching.
    let mut x = 0x1234_5678_9abc_def0u64; 
    //1234_5678_9abc_def0u64 means a 64-bit hexadecimal literal, which is a common way to initialize a variable with a specific bit pattern. The underscores are just for readability and have no effect on the value.

    for _ in 0..n {
        // xorshift-ish bit mixing
        x ^= x << 7; // ^= is a bitwise XOR assignment operator, which means x = x ^ (x << 7)
        x ^= x >> 9;
        x ^= x << 8;

        // Branch depends on changing bit pattern -> less predictable
        if (x & 1) == 0 {
            acc = acc.wrapping_add(1);
        }
    }
    acc
}

fn main() {
    let n = 150_000_000u64;

    let t1 = Instant::now();
    let a = predictable_branch(n);
    let e1 = t1.elapsed();

    let t2 = Instant::now();
    let b = unpredictable_branch(n);
    let e2 = t2.elapsed();

    println!("predictable acc={a} elapsed_ms={}", e1.as_millis());
    println!("unpredictable acc={b} elapsed_ms={}", e2.as_millis());
}
