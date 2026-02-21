/**
 * Error handling in Rust is primarily done through the `Result` and `Option` types. 
 * The `Result` type is used for functions that can return an error, 
 * while the `Option` type is used for functions that can return a value or nothing.
 */
 //Approach 1: Using Options
// enum Option<T> {
//     Some(T), // represents a value of type T
//     None, // represents the absence of a value

// }
//example of using Option
fn divide_option(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // return None if division by zero is attempted
    } else {
        Some(a / b) // return the result wrapped in Some if division is successful
    }
}


//Approach 2: Using Results
// enum Result<T, E> {
//     Ok(T), // represents a successful result containing a value of type T
//     Err(E), // represents an error containing a value of type E
// }

// example of using Result
fn divide_result(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero is not allowed")) // return an error message if division by zero is attempted
    } else {
        Ok(a / b) // return the result wrapped in Ok if division is successful
    }
}
fn main() {
    let result = divide_option(10.0, 2.0);
    match result {
        Some(value) => println!("Result: {}", value), // handle the successful case
        None => println!("Error: Division by zero is not allowed"), // handle the error case    
   }

   let result = divide_option(10.0, 5.0);
    match result {
        Some(value) => println!("Result: {}", value), // handle the successful case
        None => println!("Error: Division by zero is not allowed"), // handle the error case
    }
    // Using Result for error handling
    let result = divide_result(10.0, 0.0);
    match result {
        Ok(value) => println!("Result: {}", value), // handle the successful case
        Err(e) => println!("Error: {}", e), // handle the error case
    }
}