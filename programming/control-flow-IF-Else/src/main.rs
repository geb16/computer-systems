
// Control Flow: IF-Else
/**In Rust, you can use `if` statements to execute code based on certain conditions. 
The syntax is similar to other programming languages, but there are some unique features in Rust.
*/
fn main() {
    let age: u16 = 18;
    if age < 18 {
        println!("You are a minor.");
    } else if age == 18 {
        println!("Congratulations on reaching adulthood!"); // this part is called an arm of the if statement
    } else {
        println!("You are an adult.");
    }
    let condition: bool = true;
    let number = if condition { 5 } else { 10 }; // this is called an if expression, it returns a value
    println!("The number is: {}", number);
}
