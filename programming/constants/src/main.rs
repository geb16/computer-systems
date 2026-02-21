// -- constants--
// Constants are immutable values that are known at compile time and do not change during the execution of the program. 
// They are defined using the `const` keyword and must have a type annotation. 
//Constants can be used for values that are fixed and should not be modified, 
// such as mathematical constants or configuration values.

fn main() {
    println!("The value of PI is: {}", PI);
    println!("The speed of light is: {} m/s", SPEED_OF_LIGHT);
    println!("Three hours in seconds is: {} seconds", THREE_HOURS_IN_SECONDS);
    
}

const PI: f64 = 3.14159;
const SPEED_OF_LIGHT: u32 = 299_792_458; // in meters per
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 3 hours in seconds
