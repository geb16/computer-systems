use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100_000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("counter = {}", counter.load(Ordering::SeqCst));
}

/* 
This code creates a shared counter protected by a mutex and spawns 8 threads that 
increment the counter 100,000 times each. After all threads have finished, 
it prints the final value of the counter, which should be 800,000.
Note: The use of Arc allows multiple threads to share ownership of the counter,
while the Mutex ensures that only one thread can modify the counter at a time,
preventing race conditions.
*/