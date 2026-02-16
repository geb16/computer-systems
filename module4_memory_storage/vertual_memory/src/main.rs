use std::time::Instant;

fn main() {
    let size = 4 * 1024 * 1024 * 1024usize; // 4 GB

    println!("Allocating 4GB...");
    let mut data = vec![0u8; size];

    println!("Touching memory page by page...");

    let start = Instant::now();

    // Touch one byte every 4096 bytes (one per page)
    for i in (0..size).step_by(4096) {
        data[i] = 1;
    }

    println!("Time: {:?}", start.elapsed());
    println!("{}", std::mem::size_of::<usize>());

}
