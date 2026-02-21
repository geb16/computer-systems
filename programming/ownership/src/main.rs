// ----- Ownership in Rust -----
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// Example: Each value in Rust has a variable that’s called its owner.
// let s = String::from("Hello"); // s is the owner of the string value
// println!("{}", s); // we can use s to access the string value
// s goes out of scope here, and the string value is dropped

fn main() {
    let s1 = String::from("RUST");
    let s2 = s1; // s1 is moved to s2, and s1 is no longer the owner of the string value
    println!("{}", s2); // we can use s2 to access the string value
    // println!("{}", s1); // this will cause a compile-time error because s1 is no longer the owner of the string value
    let len = calculate_length(&s2); // we can pass a reference to s2 to the function, and it will not take ownership of the string value
    println!("The length of the {} is: {}", s2, len);
}
fn calculate_length(s: &String) -> usize {
    s.len() // we can use s to access the string value, but we cannot modify it
}