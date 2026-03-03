use sha2::{Digest, Sha256};
use std::{env, fs, process};

fn sha256_file(path: &str) -> Result<String, std::io::Error> {
    let data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let digest = hasher.finalize();
    Ok(hex::encode(digest))
}

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  integrity_checker hash <file_path>");
    eprintln!("  integrity_checker verify <file_path> <expected_sha256_hex>");
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage();
    }

    let cmd = args[1].as_str();
    let file_path = &args[2];

    match cmd {
        "hash" => {
            match sha256_file(file_path) {
                Ok(h) => println!("{}", h),
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    process::exit(1);
                }
            }
        }
        "verify" => {
            if args.len() != 4 {
                usage();
            }
            let expected = &args[3];

            match sha256_file(file_path) {
                Ok(actual) => {
                    if &actual == expected {
                        println!("OK: integrity verified");
                        process::exit(0);
                    } else {
                        println!("FAIL: file changed");
                        println!("expected: {}", expected);
                        println!("actual:   {}", actual);
                        process::exit(3);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    process::exit(1);
                }
            }
        }
        _ => usage(),
    }
}