// First we import clap derive macros.
// Parser allows struct to parse CLI.
// Subcommand allows enum variants to be CLI subcommands.
use clap::{Parser, Subcommand};

// This struct represents the entire CLI program.
// It defines global options and subcommands.
#[derive(Parser)]
#[command(name = "cli_pro")]
#[command(about = "Professional CLI Example")]
struct Cli {

    // #[command(subcommand)] means:
    // This field will parse subcommands like:
    // cli_pro enc ...
    #[command(subcommand)]
    command: Commands,
}

// This enum defines all subcommands.
// Each variant becomes a CLI command.
#[derive(Subcommand)]
enum Commands {

    // Example:
    // cli_pro enc key.bin message.txt message.enc
    Enc {
        key_file: String,
        in_file: String,
        out_file: String,
    },

    // Example:
    // cli_pro hash file.txt
    Hash {
        file: String,
    },
}

fn main() {

    // Parse CLI arguments into typed struct
    let cli = Cli::parse();

    // Match on the parsed enum
    match cli.command {

        Commands::Enc { key_file, in_file, out_file } => {
            println!("ENC called");
            println!("key: {}", key_file);
            println!("input: {}", in_file);
            println!("output: {}", out_file);
        }

        Commands::Hash { file } => {
            println!("HASH called");
            println!("file: {}", file);
        }
    }
}