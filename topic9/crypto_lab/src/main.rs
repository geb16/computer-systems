// first we import clap for CLI parsing.
// It lets us define subcommands like `hash`, `enc`, `tls-info` safely.
use clap::{Parser, Subcommand};

// then we import filesystem and I/O helpers
// - fs::read / fs::write for file bytes
// - Read/Write traits not strictly needed here (we do simple read/write)
use std::fs;

// then we import SHA-256 hasher
use sha2::{Digest, Sha256};

// then we import AES-GCM AEAD types.
// aes_gcm provides:
// - Aes256Gcm: the cipher (AES-256 in GCM mode)
// - Key / Nonce types
// - aead::Aead trait: encrypt/decrypt interface
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};

// then we import rand for secure random bytes (keys + nonces)
use rand::rngs::OsRng;
use rand::RngCore;

// then we import TLS client components.
// rustls does the real TLS handshake in userspace.
// webpki_roots provides a standard set of public trust roots for verification.
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use webpki_roots::TLS_SERVER_ROOTS;

// then we import TCP stream for connecting to a server (TLS runs over TCP)
use std::net::TcpStream;
use std::sync::Arc;

// -----------------------------
// 1) CLI definitions
// -----------------------------

// We define the top-level CLI structure.
// Think: "what commands does our tool support?"
#[derive(Parser, Debug)]
#[command(name = "crypto_lab", about = "SHA-256 integrity, AES-GCM encryption, TLS inspection")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

// Each subcommand is one capability.
// This is better than manual argv parsing (less bug-prone).
#[derive(Subcommand, Debug)]
enum Command {
    // SHA-256: compute hash of a file
    Hash {
        file: String,
    },

    // SHA-256: verify file matches expected hex digest
    Verify {
        file: String,
        expected_hex: String,
    },

    // Generate a new random 32-byte key and store in a file (raw bytes)
    Genkey {
        key_file: String,
    },

    // Encrypt a file using AES-256-GCM
    // Output format: [12-byte nonce][ciphertext+tag]
    Enc {
        key_file: String,
        in_file: String,
        out_file: String,
    },

    // Decrypt a file using AES-256-GCM
    Dec {
        key_file: String,
        in_file: String,
        out_file: String,
    },

    // TLS mental model verification: connect, handshake, print negotiated params and cert fingerprint
    TlsInfo {
        host: String, // e.g. "example.com"
        port: u16,    // e.g. 443
    },
}

// -----------------------------
// 2) SHA-256 integrity functions
// -----------------------------

// we need a function to compute SHA-256 of bytes.
fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

// we need a function to compute SHA-256 of a file (read as bytes).
fn sha256_file(path: &str) -> Result<[u8; 32], std::io::Error> {
    let data = fs::read(path)?;
    Ok(sha256_bytes(&data))
}

// -----------------------------
// 3) AES-GCM (AEAD) functions
// -----------------------------

// we need a function to load a 32-byte key from a file.
// For learning: the key file is raw 32 bytes.
fn load_key_32(path: &str) -> Result<[u8; 32], String> {
    let key_bytes = fs::read(path).map_err(|e| format!("Failed to read key file: {e}"))?;
    if key_bytes.len() != 32 {
        return Err(format!(
            "Key file must be exactly 32 bytes for AES-256-GCM, got {} bytes",
            key_bytes.len()
        ));
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&key_bytes);
    Ok(key)
}

// we need a function to encrypt a file using AES-256-GCM with safe nonce handling.
// SAFE NONCE RULE (practical):
// - For GCM, never reuse the same (key, nonce) pair.
// - Easiest safe approach: generate a fresh random 12-byte nonce for every encryption.
fn aes_gcm_encrypt(key_32: [u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    // create cipher instance from key
    let key = Key::<Aes256Gcm>::from_slice(&key_32);
    let cipher = Aes256Gcm::new(key);

    // generate random 96-bit nonce (12 bytes) - standard size for GCM
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // encrypt returns ciphertext with authentication tag appended internally
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| "Encryption failed (AEAD error)".to_string())?;

    // output layout: [nonce][ciphertext+tag]
    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

// we need a function to decrypt.
// It expects input layout: [12-byte nonce][ciphertext+tag]
fn aes_gcm_decrypt(key_32: [u8; 32], input: &[u8]) -> Result<Vec<u8>, String> {
    if input.len() < 12 {
        return Err("Input too short: missing 12-byte nonce".to_string());
    }

    let (nonce_bytes, ciphertext) = input.split_at(12);

    let key = Key::<Aes256Gcm>::from_slice(&key_32);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed: data tampered OR wrong key/nonce".to_string())?;

    Ok(plaintext)
}

// -----------------------------
// 4) TLS mental-model verifier (rustls)
// -----------------------------

// we need a function to build a rustls client config with standard root CAs.
fn build_tls_config() -> ClientConfig {
    let mut roots = RootCertStore::empty();
    // Load well-known public roots (for server cert verification).
    roots.extend(TLS_SERVER_ROOTS.iter().cloned());

    ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth()
}

// we need a function to connect, do TLS handshake, and print negotiated info.
// This is the mental model:
// - TCP connect
// - TLS handshake: negotiate version + cipher suite, validate certificate chain
// - then we can inspect: protocol version, cipher, cert fingerprint
fn tls_info(host: &str, port: u16) -> Result<(), String> {
    // TCP connect first (TLS runs over the TCP stream)
    let addr = format!("{host}:{port}");
    let mut tcp = TcpStream::connect(&addr).map_err(|e| format!("TCP connect failed: {e}"))?;

    // Build TLS config and connection
    let config = Arc::new(build_tls_config());

    // ServerName is required for SNI + certificate verification.
    let server_name = host
        .to_string()
        .try_into()
        .map_err(|_| "Invalid DNS name for TLS ServerName".to_string())?;

    let mut conn =
        ClientConnection::new(config, server_name).map_err(|e| format!("TLS init failed: {e}"))?;

    // Complete handshake by repeatedly calling complete_io.
    // rustls handles the protocol; we just drive I/O.
    while conn.is_handshaking() {
        conn.complete_io(&mut tcp)
            .map_err(|e| format!("TLS handshake I/O failed: {e}"))?;
    }

    // After handshake, we can inspect connection state.
    let protocol = conn
        .protocol_version()
        .map(|v| format!("{v:?}"))
        .unwrap_or_else(|| "Unknown".to_string());

    let cipher = conn
        .negotiated_cipher_suite()
        .map(|cs| format!("{:?}", cs.suite()))
        .unwrap_or_else(|| "Unknown".to_string());

    // Get peer cert chain (DER). Compute SHA-256 fingerprint of leaf cert.
    let certs = conn
        .peer_certificates()
        .ok_or("No peer certificates presented".to_string())?;

    let leaf = certs
        .first()
        .ok_or("Empty certificate chain".to_string())?
        .as_ref();

    let fp = sha256_bytes(leaf);
    let fp_hex = hex::encode(fp);

    println!("TLS connected to {host}:{port}");
    println!("  negotiated protocol: {protocol}");
    println!("  negotiated cipher:   {cipher}");
    println!("  cert chain length:   {}", certs.len());
    println!("  leaf cert sha256:    {fp_hex}");

    Ok(())
}

// -----------------------------
// 5) main: parse CLI and dispatch
// -----------------------------

fn main() {
    // parse CLI args into our strongly-typed command struct
    let cli = Cli::parse();

    match cli.cmd {
        Command::Hash { file } => {
            // compute hash and print hex
            match sha256_file(&file) {
                Ok(d) => println!("{}", hex::encode(d)),
                Err(e) => {
                    eprintln!("Error hashing file: {e}");
                    std::process::exit(1);
                }
            }
        }

        Command::Verify { file, expected_hex } => {
            match sha256_file(&file) {
                Ok(d) => {
                    let actual_hex = hex::encode(d);
                    if actual_hex.eq_ignore_ascii_case(&expected_hex) {
                        println!("OK: integrity verified");
                    } else {
                        println!("FAIL: file changed");
                        println!("expected: {expected_hex}");
                        println!("actual:   {actual_hex}");
                        std::process::exit(2);
                    }
                }
                Err(e) => {
                    eprintln!("Error hashing file: {e}");
                    std::process::exit(1);
                }
            }
        }

        Command::Genkey { key_file } => {
            // generate 32 random bytes and write to file with restrictive permissions recommendation
            let mut key = [0u8; 32];
            OsRng.fill_bytes(&mut key);

            if let Err(e) = fs::write(&key_file, &key) {
                eprintln!("Failed to write key file: {e}");
                std::process::exit(1);
            }

            println!("Wrote 32-byte AES-256-GCM key to {key_file}");
            println!("Ubuntu hardening tip: chmod 600 {key_file}");
        }

        Command::Enc {
            key_file,
            in_file,
            out_file,
        } => {
            let key = match load_key_32(&key_file) {
                Ok(k) => k,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            };

            let plaintext = match fs::read(&in_file) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Failed to read input file: {e}");
                    std::process::exit(1);
                }
            };

            let out = match aes_gcm_encrypt(key, &plaintext) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            };

            if let Err(e) = fs::write(&out_file, &out) {
                eprintln!("Failed to write output file: {e}");
                std::process::exit(1);
            }

            println!("Encrypted {in_file} -> {out_file}");
            println!("Format: [12-byte nonce][ciphertext+tag]");
        }

        Command::Dec {
            key_file,
            in_file,
            out_file,
        } => {
            let key = match load_key_32(&key_file) {
                Ok(k) => k,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            };

            let input = match fs::read(&in_file) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Failed to read encrypted file: {e}");
                    std::process::exit(1);
                }
            };

            let plaintext = match aes_gcm_decrypt(key, &input) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            };

            if let Err(e) = fs::write(&out_file, &plaintext) {
                eprintln!("Failed to write decrypted file: {e}");
                std::process::exit(1);
            }

            println!("Decrypted {in_file} -> {out_file}");
        }

        Command::TlsInfo { host, port } => {
            if let Err(e) = tls_info(&host, port) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
}