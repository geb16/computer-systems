// We import HashSet to compare process lists efficiently
use std::collections::HashSet;

// Define a struct representing a process

#[allow(dead_code)]
struct Process {
    pid: u32,
    name: String,
}

// Function to simulate kernel process list (ground truth)
fn kernel_process_list() -> Vec<Process> {
    vec![
        Process { pid: 1, name: "init".to_string() },
        Process { pid: 100, name: "nginx".to_string() },
        Process { pid: 666, name: "malware_agent".to_string() }, // hidden process
    ]
}

// Function to simulate user-space `ps` output
// Rootkit hides pid 666
fn user_process_list() -> Vec<Process> {
    vec![
        Process { pid: 1, name: "init".to_string() },
        Process { pid: 100, name: "nginx".to_string() },
        // malware_agent missing
    ]
}

// Detection function using cross-view comparison
fn detect_hidden_processes() {
    let kernel = kernel_process_list();
    let user = user_process_list();

    let kernel_pids: HashSet<u32> = kernel.iter().map(|p| p.pid).collect();
    let user_pids: HashSet<u32> = user.iter().map(|p| p.pid).collect();

    for pid in kernel_pids.difference(&user_pids) {
        println!("⚠ Suspicious: PID {} exists in kernel but not visible to user-space", pid);
    }
}

fn main() {
    println!("Running rootkit cross-view detection simulation...\n");
    detect_hidden_processes();
}

//1️⃣ Why are kernel rootkits harder to detect than user-space rootkits?
// Kernel rootkits are harder to detect because they operate at a lower privilege level (kernel mode) and can directly manipulate kernel data structures, making them invisible to user-space tools like `ps` or `lsof`.
//2️⃣ What does cross-view detection compare?
// Cross-view detection compares process lists from different system views (e.g., kernel vs. user-space) to identify discrepancies that may indicate hidden processes.
//3️⃣ Why is file integrity monitoring useful against rootkits?
// File integrity monitoring helps detect rootkits by tracking changes to critical system files and identifying unauthorized modifications that may be signs of rootkit presence.