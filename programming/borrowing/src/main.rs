// --- Borrowing & References ---
// Mutable Refernce
// Create Refrence by adding "&" before the variable name
// Mutable Refrence by adding "mut" before the variable name
// Mutable Refrence can only have one refrence at a time
// Mutable Refrence can not be used while there is an immutable refrence
// Immutable Refrence can have multiple refrences at a time
// Immutable Refrence can not be used while there is a mutable refrence
// example of borrowing: 
// let x = 5;
// let y = &x; // y is a reference to x, we are borrowing x
// println!("The value of x is: {}", x);
// println!("The value of y is: {}", y); // we can use y to access the value of x, but we cannot change it because y is an immutable reference

// -I- Immutable Refrence

fn main() {
    let mut _x: i32 = 5;
    let _y: &mut i32 = &mut _x; // Mutable Refrence
    *_y+= 1; // this is how we can change the value of x using the mutable reference y
    *_y-=3;
    
    println!("The value of x is: {}", _x);
    //println!("The value of y is: {}", _y);
}