fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world");
    s.push('!');
    let mut bytes = s.as_bytes();
    // bytes to chars from bytes = s.as_bytes(); back to string from bytes = s.as_bytes(); 
    let s2 = String::from_utf8(bytes.to_vec());


    println!("{s}");
    println!("world = {}", &s[2..4]); 
    println!("bytes = {:?}", bytes);
    //println!("chars = {:?}", chars);
    println!("s2 = {:?}", s2);
    // why the output is "ll" instead of "world"? because the index is based on bytes, not characters.
    //based on byte meaning that each character may take up more than one byte, so the index is not based on characters but on bytes.
    // let c = s[0]; // this will cause an error because s[0] is not a valid index, it is based on bytes, not characters.
    // but  let in bytes = s.as_bytes(); let c = bytes[0]; will work because it is based on bytes, not characters.
}
