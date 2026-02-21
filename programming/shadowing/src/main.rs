// This is Shadowing

fn main() {
    let x = 5;
    let x = x + 1; // shadowing the previous x
    let x = x * 2; // shadowing again

    { 
        let x = x +3; // shadowing in a new scope
        println!("The value of x in the inner scope is: {}", x); // prints 9
     }
    println!("The value of x in the outer scope is: {}", x); // prints 12
}


