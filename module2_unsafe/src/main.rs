fn main() {
    let x = 10;
    let ptr = &x as *const i32; // raw pointer

    unsafe {
        println!("{}", *ptr);  // manual dereference
    }
}
