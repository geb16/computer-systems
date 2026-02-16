fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(10);
    v.push(20);
    v.push(30);

    println!("vector = {:?}", v);
    println!("first = {}", v[0]);
    println!("{}", v[0]);
    println!("second = {}", v[1]);
    println!("third = {}", v[2]);
    //println!("last = {}", v[4]);
    println!("length = {}", v.len());
    if let Some(x) = v.get(100) {
        println!("{x}");
    } else {
        println!("index out of bounds");
    }
}
/**
 * Vectors vs Hashmaps vs Strings
 *  Vectors are ordered collections of values, where each value is identified by its index.
 * Hashmaps are unordered collections of key-value pairs, where each value is identified by its key.
 * Strings are collections of characters, where each character is identified by its index in the string.
 * Vectors are useful when you want to store a list of values, and you want to access them by their index.
 * Hashmaps are useful when you want to store a collection of key-value pairs, and you want to access them 
 * by their key.
 * Strings are useful when you want to store a sequence of characters, and you want to access them by their index in the string.
 * Arrays are similar to vectors, but they have a fixed size, and they are stored on the stack, while vectors are stored on the heap. 
 * Arrays are useful when you know the size of the collection at compile time, and you want to store it on the stack for better performance. 
 * Vectors are useful when you don't know the size of the collection at compile time, 
 */
