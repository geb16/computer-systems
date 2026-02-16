//Ownership Violation #1 — Use After Move
// fn main() {
//     let s = String::from("data");
//     let t = s;              // ownership moves here

//     println!("{s}");        // ❌ forbidden
// }

// --------------------------------
// Ownership Violation #2 — Multiple Mutable Access
// fn main() {
//     let mut s = String::from("Hello");

//     let r1 = &mut s;       // first mutable reference
//     //let r2 = &mut s;       // ❌ second mutable reference

//     println!("{r1}");
// }

// --------------------------------
// Ownership Violation #3 — Dangling Reference
fn main() {
    let r: &String;
    {
        let mut s = String::from("hello");
        r = &mut s;                 // r points to s (reference escapes the scope)
        //println!("{r}");      // r would be fine here
    }                           // s goes out of scope here
    println!("{r}");         // ❌ r is dangling
}


// --------------------------------
// rustc --explain E0597
// This error occurs because a value was dropped while it was still borrowed.
// struct Foo<'a> {
//     x: Option<&'a u32>,
// }
// fn main() {
//     let mut x = Foo { x: None };

//     let y = 0;
//     x.x = Some(&y);

//     println!("{:?}", x.x);
// }