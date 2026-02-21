

fn main() {
    hello_world();
    let sum = add(5, 10);
    println!("The sum of 5 and 10 is: {}", sum);
    let person_info = human_id("Alice", 30, 165.0);
    println!("{}", person_info);

    // Expressions an be in code blocks, and they can be used to assign values to variables
    let x: i32 = {
        let price = 10;
        let qty = 20;
        price * qty // this is the last expression in the block, and its value will be assigned to x
    };
    println!("Result is: {}", x);

    //add (3, 4); 
    let y = add (3, 4); // this is an expression that evaluates to 7, and its value is assigned to y
    println!("The value of y is: {}", y);
    // allternatively, we can also write it as:
    println!("The value of y from function is: {}", add(3, 4)); // here we are directly using the expression in the println! macro
    // Calling the BMI calculator
    let weight = 70.0; // in kg
    let height = 1.75; // in meters
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi); // 
}

// Hoisting:  can calla function  anwyhere in the code, even before its declaration
fn hello_world(){
    println!("Hello, Rust🦀");
} 
// we can insert more than value in the function, we can also insert parameters
fn add(a: i32, b: i32) -> i32 {
    a + b 
}
fn  human_id(name: &str, age: u32, height: f32) -> String {
    format!("My name is {}, I am {} years old,  and my height is  {} cm", name, age, height)
}

// Expressions and Statements

// Expressions:  produce a value and can be part of a statement
// example: let x = 5 + 3; // 5 + 3 is an expression that evaluates to 8
//   add  (3, 4)
//  (True && False) || (False && True)

// Statements: perform an action but do not produce a value
// example: let x = 5 + 3; // the entire line is a statement


// Final example: 
// the BMI (Body Mass Index) calculator
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg  / (height_m * height_m)

    // or you can also write it as:
    // weight_kg / height_m.powi(2)
    //fn calculate_bmi(weight_kg: f32, height_m: f32) -> f64 {
    //   weight_kg  as f64/ (height_m * height_m) as f64
}
