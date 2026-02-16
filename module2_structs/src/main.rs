// Define a struct (a memory layout)
struct Process {
    pid: u32,        // process ID
    memory_kb: u64,  // memory usage
    running: bool,   // state flag
}

fn main() {
    // Instantiate the struct
    let p1 = Process {
        pid: 1001,
        memory_kb: 20480, // 20480 kb 
        running: true,
    };

    println!(
        "pid={} memory={}KB running={}",
        p1.pid, p1.memory_kb, p1.running
    );
}

// cargo run --bin module2_ownership  --bin means: run the binary named module2_ownership
// cargo run --release --bin module2_ownership  --release means: run the optimized version of the binary named module2_ownership