use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let _name = String::from("Alice");
    //scores.insert(name, 10);
    //println!("name = {}", name); // this will cause an error because name has been moved into the hashmap, 
    //so it cannot be used anymore.
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);
    scores.insert("Charlie", 30);

    for (name, score) in &scores {
        println!("{name}: {score}");
    }
    // to print all the scores
    for score in scores.values() {
        println!("score = {score}");
    }
    // to print all the names
    for name in scores.keys() {
        println!("name = {name}");
    }
}   
