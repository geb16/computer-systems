fn main() {
    let s = String::from("hi");

    let r1 = &s; // & means immutable reference
    let r2 = &s; // another immutable reference
    //let r3 = &mut s; // <-- should fail why

    println!("{r1} {r2}");
}
