//---- Variables and Mutability in Rust ----

fn main() {
    // declaring an immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    
    // trying to change the value of x will cause a compile-time error
    // x = 10;  --- IGNORE ---
    
    // declaring a mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    
    // changing the value of y
    y = 20;
    println!("The value of y after mutation is: {}", y);
}