use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
/*
    Mutex: A mutex (short for "mutual exclusion") is a synchronization primitive that allows only one thread 
    to access a resource at a time. It provides a lock mechanism to ensure that only one thread can access 
    the shared resource, preventing data corruption and ensuring thread safety.
    
    Arc: Arc (Atomic Reference Counted) is a thread-safe reference-counting pointer. 
    It allows multiple threads to share ownership of a resource safely. 
    Arc is used to enable shared ownership of data across threads, and it automatically 
    manages the reference count to ensure that the resource is deallocated when no longer needed.
*/
fn main() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0)); 

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);

    let t1 = thread::spawn(move || {
        let _lock_a = a1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let _lock_b = b1.lock().unwrap();
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);

    let t2 = thread::spawn(move || {
        let _lock_b = b2.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let _lock_a = a2.lock().unwrap();
    });

    t1.join().unwrap();
    //t2.join().unwrap();
}

/*
In this code, we have two threads (t1 and t2) that are trying to acquire locks on two shared resources (a and b).

Thread t1 acquires lock on resource a, then sleeps for 100ms, and then tries to acquire lock on resource b.
Thread t2 acquires lock on resource b, then sleeps for 100ms, and then tries to acquire lock on resource a.

This creates a deadlock situation because:
1. t1 holds the lock on a and waits for b.
2. t2 holds the lock on b and waits for a.
3. Neither thread can proceed because each is waiting for the other's resource.

This is a classic deadlock scenario in concurrent programming.
thus, when you run this code, it will hang indefinitely as both threads are waiting for each other 
to release the locks they hold.
So the output of this code will be that it will not produce any output and will 
hang indefinitely due to the deadlock.
*/