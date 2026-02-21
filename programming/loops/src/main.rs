// This is loops in Rust
/**In Rust, loops are used to execute a block of code repeatedly. 
 * There are three types of loops in Rust: `loop`, `while`, and `for`. 
 * Each type serves a different purpose and has its own syntax and use cases.
 * 1. `loop`: This is an infinite loop that will continue to execute until it is 
 * explicitly broken out of using the `break` statement.
 * 2. `while`: This loop continues to execute as long as a specified condition is true.
 * 3. `for`: This loop is used to iterate over a range of values or collections, such as arrays or vectors.
 */
fn main() {
    // Example of a `loop`
    let mut counter = 0;
    let result = loop {
        counter += 1;
        //println!("Count: {}", counter);
        if counter == 20 {
            break counter*2; // Exit the loop when count reaches 20 and return the value of counter*2
        }
    }; 
    println!("The result is: {}", result); 

    // Loop labels and nested loops
    let mut count = 0;
    'outer: loop {
        println!("Outer loop count: {}", count);
        let mut inner_count = 0;
        loop {
            println!("  Inner loop count: {}", inner_count);
            if inner_count == 2 {
                break; // Breaks the inner loop
            }
            if count == 3 {
                break 'outer; // Breaks the outer loop
            }
            inner_count += 1;
        }
        count += 1;
    }
    // // Example of a `while` loop
    // let mut number = 0;
    // while number < 5 {
    //     println!("Number: {}", number);
    //     number += 1;
    // }
    // // Example of a `for` loop
    // let array = [10, 20, 30, 40, 50];
    // for element in array.iter() {
    //     println!("Element: {}", element);
    // }
    // Using a `for` loop with a range
    for i in 0..5 {
        println!("i: {}", i);
    }

    // Using a `for` loop with a range and step
    for i in (0..10).step_by(2) { //stepping by 2, so it will print 0, 2, 4, 6, 8
        println!("i: {}", i);
    }

    let a = [1, 2, 3, 4, 5];
    for (index, value) in a.iter().enumerate() { //enumerate gives us the index and the value of each element in the array
        println!("Index: {}, Value: {}", index, value);
    }
    let cities = vec!["New York", "Los Angeles", "Chicago"];
    for city in &cities { //iterating over a vector of strings
        println!("City: {}", city);
    }
}