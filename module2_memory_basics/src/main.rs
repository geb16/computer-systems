fn main() {
    let x = 5;                     // stack
    let s = String::from("hello"); // heap

    print(&s);                     // borrow, no ownership transfer . to transfer ownership, use &mut for mutable references like: let s_mut = &mut s;
    println!("{x}");
}

fn print(data: &String) {
    println!("{data}");
}
