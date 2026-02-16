use std::time::Instant;

fn main() {
    // Instant = monotonic clock suitable for performance timing.
    // Monotonic means it won't go backwards due to system time changes.
    let start = Instant::now();

    // A deterministic CPU workload: sum integers.
    // This mostly exercises CPU ALU + registers + loop control.
    // ALU = Arithmetic Logic Unit, the part of the CPU that does math and logic operations.
    let mut sum: u64 = 0; //  u64 = 64-bit unsigned integer means
    for i in 0..200_000_000u64 { //0..200_000_000u64 means: from 0 up to (not including) 200 million as u64
        sum = sum.wrapping_add(i);
    }

    let elapsed = start.elapsed();
    println!("sum={sum} elapsed_ns={}", elapsed.as_nanos());
}

// i32vs u64:
// i32 is 32-bit signed integer: -2,147,483,648 to 2,147,483,647
// u64 is 64-bit unsigned integer:0 to 18,446,744,073,709,551,615