use std::env;

fn some_function() -> u64 {
    42
}

fn main() {
    // 1) CODE address (function pointer) => virtual address in the "text/code" region
    let code_va = some_function as usize;

    // 2) STACK address (local variable lives on stack)
    let stack_local: u64 = 123;
    let stack_va = (&stack_local as *const u64) as usize;

    // 3) HEAP address (Box allocates on heap)
    let heap_box = Box::new(999u64);
    let heap_va = (&*heap_box as *const u64) as usize;

    // 4) ARG/ENV memory (strings typically allocated on heap by Rust/OS runtime)
    let args: Vec<String> = env::args().collect();
    let args_va = args.as_ptr() as usize;

    println!("Virtual Address demo (these are VAs, not physical addresses)");
    println!("code_va  = 0x{code_va:016x}  (function pointer / code)");
    println!("stack_va = 0x{stack_va:016x}  (local variable / stack)");
    println!("heap_va  = 0x{heap_va:016x}  (Box<u64> / heap)");
    println!("args_va  = 0x{args_va:016x}  (Vec<String> buffer / heap-ish)");

    // Keep values used so optimizer doesn't erase them.
    println!("values: stack_local={stack_local}, heap_box={}, func={}", *heap_box, some_function());
}
