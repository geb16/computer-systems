// Collection Types in Rust
/*
 * Rust provides several built-in collection types,
 * Vectors - UTF - Hashmaps
 * Vectors are resizable arrays that can hold values of the same type.
 * Strings are UTF-8 encoded, growable, and owned string types.
 * Hash Maps are collections of key-value pairs where each key maps to a value.
*/
fn main(){
    // Vectors
    // let mut _v: Vec<i32> = Vec::new();
    // let mut _v = vec![1, 2, 3];  // Using the vec! macro to create a vector with initial values
    // _v.push(4);
    // _v.push(5);
    // _v.push(6);
    // println!("Vector: {:?}", _v);
    
    let mut _v = vec![1, 2, 3];  // Using the vec! macro to create a vector with initial values 
    
    let third = &_v[2];  // Direct indexing (panics if out of bounds)
    println!("The third element is: {third}");
    // th output of ({:?}) is Some(3) because the get method returns an Option type, which can be either Some(value) if the index exists or None if it doesn't. In this case, since the index 2 exists in the vector, it returns Some(3).
    match _v.get(2) {
        Some(value) => println!("The third element from get method is: {}", value),
        None => println!("No third element exists"),
    }

    // -------------------UTF-8 Strings--------------------
    #[allow(unused_variables)]
    let  s = String::new();
    // 2
    #[allow(unused_variables)]
    let s  = String ::from ("Welcome to Rust!");
    //  Mutate the variable [push_Str to it]
    let mut s = String::from("Hello");
    s.push_str(", world");
    s.push('!');  // Adding a single character to the string
    println!("UTF-8 String: {}", s);

    // concatenation us the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // Note that s1 is moved here and can no longer be used
    println!("Concatenated String: {}", s3);

    // Formating Strings
    let salam = String::from("Arabic");
    let salut = String::from("French");
    let full_message = format!("{} {}!", salut, salam);
    println!("{full_message}");
    
    //----------- Hash Maps -----------------
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    println!("Hash Map: {:?}", scores);
    println!("Alice's score: {}", scores.get("Alice").unwrap());
}