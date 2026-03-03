// // first we import std::env
// // this allows us to access command-line arguments
// use std::env;

// fn main() {
//     // env::args() returns an iterator over CLI arguments
//     // collect() converts iterator into a Vec<String>
//     let args: Vec<String> = env::args().collect();

//     println!("All args: {:?}", args);

//     // args[0] = program name
//     // args[1] = first argument
//     // etc.

//     if args.len() < 2 {
//         println!("Usage: cli_basic <command>");
//         return;
//     }

//     let command = &args[1];

//     match command.as_str() {
//         "hello" => {
//             println!("Hello command executed");
//         }
//         "bye" => {
//             println!("Bye command executed");
//         }
//         _ => {
//             println!("Unknown command");
//         }
//     }
// }
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cli_basic <command>");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "enc" => {
            if args.len() != 5 {
                println!("Usage: enc <key_file> <in_file> <out_file>");
                return;
            }

            let key_file = &args[2];
            let in_file = &args[3];
            let out_file = &args[4];

            println!("Encrypting:");
            println!("  key: {}", key_file);
            println!("  input: {}", in_file);
            println!("  output: {}", out_file);
        }

        _ => {
            println!("Unknown command");
        }
    }
}