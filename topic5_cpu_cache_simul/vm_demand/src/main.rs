fn main() {
    // Allocate 500 MB but DO NOT touch it
    let size = 500 * 1024 * 1024; // 1024 * 1024 * 1024 = 1 MB
    // 1KB = 1024 bytes, 1MB = 1024 KB, so 500MB = 500 * 1024 * 1024 bytes
    let v = vec![0u8; size];

    println!("Allocated 500MB. Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();

    // Now touch each page (force allocation)
    for i in (0..size).step_by(4096) {
        unsafe {
            let ptr = v.as_ptr() as *mut u8;
            *ptr.add(i) = 1;
        }
    }

    println!("Touched all pages. Press Enter to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
