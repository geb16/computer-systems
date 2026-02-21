use std::mem::{size_of, align_of};

#[derive(Debug)]
struct Simple {
    a: u8,
    b: u32,
}

#[derive(Debug)]
struct Optimized {
    b: u32,
    a: u8,
}

#[derive(Debug)]
enum Status {
    Idle,
    Running(u32),
    Error(u8),
}

fn main() {
    println!("--- Basic Type Sizes ---");
    println!("u8 size: {} bytes", size_of::<u8>());
    println!("u32 size: {} bytes", size_of::<u32>());
    println!("usize size: {} bytes", size_of::<usize>());
    println!("&u8 size: {} bytes", size_of::<&u8>());

    println!("\n--- Struct Layout ---");
    println!("Simple size: {}", size_of::<Simple>());
    println!("Simple alignment: {}", align_of::<Simple>());
    println!("Optimized size: {}", size_of::<Optimized>());
    println!("Optimized alignment: {}", align_of::<Optimized>());

    let sample_simple = Simple { a: 7, b: 0xDEAD_BEEF };
    let sample_optimized = Optimized { b: 42, a: 1 };
    println!(
        "Simple demo => a: {}, b: {} (shows padding cost)",
        sample_simple.a, sample_simple.b
    );
    println!(
        "Optimized demo => a: {}, b: {} (tighter packing)",
        sample_optimized.a, sample_optimized.b
    );

    println!("\n--- Enum Layout ---");
    println!("Status size: {}", size_of::<Status>());
    println!("Status alignment: {}", align_of::<Status>());

    let states = [Status::Idle, Status::Running(128), Status::Error(2)];
    for state in &states {
        match state {
            Status::Idle => println!("State: Idle"),
            Status::Running(ticks) => println!("State: Running with {ticks} ticks"),
            Status::Error(code) => println!("State: Error code {code}"),
        }
    }

    println!("\n--- Option Optimization ---");
    println!("Option<&u8> size: {}", size_of::<Option<&u8>>());
    println!("&u8 size: {}", size_of::<&u8>());
}
