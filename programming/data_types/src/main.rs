//Compound Data Types:  can store multiple values of different types in a single variable
// 1. Tuples:  a fixed-size collection of values of different types
// example: let person: (&str, u32, f32) = ("Alice", 30, 165.0);
// 2. Arrays:  a fixed-size collection of values of the same type( homogeneous) 
// example: let numbers: [i32; 5] = [1, 2, 3, 4, 5];
// 3. Structs:  a custom data type that can store multiple values of different types, and it can also have methods associated with it
// example: 
// struct Person {
//     name: String,
//     age: u32,
//     height: f32,
// }
// let person = Person {
//     name: String::from("Alice"),
//     age: 30,
//     height: 165.0,
// };
// 4. Enums:  a custom data type that can store multiple values of different types, but it can only be one of those types at a time
// example: 
// enum Shape {
//     Circle(f32), // radius
//     Rectangle(f32, f32), // width, height
//     Triangle(f32, f32, f32), // sides
// }
// let shape = Shape::Circle(10.0);     

// Arrays
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The first number is: {}", numbers[0]);
    println!("The length of the array is: {}", numbers.len());
    println!("The sum of the numbers is: {}", numbers.iter().sum::<i32>());

    let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"];
    println!("The first fruit is: {}", fruits[0]);
    // in debuggable format
    println!("The fruits are: {:?}", fruits);

    // Tuples
    let person: (&str, u32, f32) = ("Alice", 30, 165.0);
    println!("The person's name is: {}", person.0);
    println!("The person's age is: {}", person.1);

    let person:(String, u32, bool)  = (String::from("Alice"), 30, true);
    println!("The person's name is: {}", person.0);
    println!("The person's age is: {}", person.1);
     

    // Slices
    let numbers_slices:&[i32] = &[1, 2, 3, 4, 5];
    println!("Numbers slice: {:?}", numbers_slices);

    let animal_slices:&[&str] = &["Dog", "Cat", "Bird", "Fish", "Horse"];
    println!("Animal slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"The Great Gatsby".to_string(), 
    &"To Kill a Mockingbird".to_string(), &"1984".to_string(), 
    &"Pride and Prejudice".to_string(), &"The Catcher in the Rye".to_string()];
    println!("Book slice: {:?}", book_slices);


    // Strings vs String Slices(&str)
    // Strings [ growable, mutable owned, heap-allocated] vs String Slices(&str) [immutable, borrowed, stack-allocated]
    // String literals are immutable and stored in the binary, while String objects are mutable and stored on the heap
    // String literals are of type &str, while String objects are of type String
    // String literals are more efficient for small, static strings, while String objects are more flexible and can be modified at runtime
    
    let string_literal: &str = "Hello, world!";
    let mut string_object: String = String::from("Hello, world!");
    string_object.push_str(" Welcome to Rust programming.");
    //string_literal.push_str(" Welcome to Rust programming."); // this will cause an error because string literals are immutable
    println!("String literal: {}", string_literal);
    println!("String object: {}", string_object);

    // B-&str (string Slice) : borrowed reference to a string, it is immutable and stored on the stack
    let string_slice: &str = "Hello, Rust!";
    println!("String slice: {}", string_slice);

    let string_object2: String = String::from("Hello, Rust!");
    let slice = &string_object2[0..5]; // this is a string slice that borrows a portion of the string object
    println!("String slice from string object: {}", slice);
}   

// I rust whe have to types of  formats:
// 1. Debuggable format:  used for debugging purposes, and it is represented by the {:?} placeholder in the format string
// example: println!("The person is: {:?}", person);
// 2. Display format:  used for user-facing output, and it is represented by the {} placeholder in the format string
// example: println!("The person is: {}", person);