// first we import the HTTP server types
use tiny_http::{Header, Response, Server, StatusCode};

// then we import filesystem for reading file bytes
use std::fs;

// then we import path utilities:
// - Path and PathBuf are Rust's safe filesystem path types
use std::path::{Path, PathBuf};

// then we import env so we can optionally control the base directory later (extension idea)
use std::env;

// then we need a helper function to safely turn a URL path into a file path
// without allowing directory traversal.
//
// function purpose:
// - input: base directory (e.g. "./public") and requested URL (e.g. "/index.html")
// - output: Some(canonical_path) if safe, None if unsafe or invalid
fn safe_path(base: &Path, url_path: &str) -> Option<PathBuf> {
    // remove leading '/' so join() treats it as relative
    // "/index.html" -> "index.html"
    let relative = url_path.trim_start_matches('/');

    // join base + relative
    // base="./public", relative="index.html" -> "./public/index.html"
    let joined = base.join(relative);

    // canonicalize resolves:
    // - ".." and "."
    // - symlinks (important!)
    // If canonicalize fails (file not found, permission), return None
    let canonical = joined.canonicalize().ok()?;
    // e.g. "./public/../secret.txt" -> "/home/user/secret.txt"
    // so canonicalize() protects against directory traversal by resolving the path first.

    // SECURITY CHECK:
    // Only allow if the final resolved path is still under base
    // If someone tries "../../etc/passwd", canonical will point outside base.
    if canonical.starts_with(base) {
        Some(canonical)
    } else {
        None
    }
}

// we need a small helper function to produce a plain-text response with a status code
fn text_response(status: StatusCode, body: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut resp = Response::from_string(body).with_status_code(status);

    // add explicit Content-Type so clients treat it as text
    let header = Header::from_bytes("Content-Type", "text/plain; charset=utf-8").unwrap();
    resp.add_header(header);

    resp
}

fn main() {
    // base directory: this is the only place we will serve files from
    // We'll default to "./public"
    let base_dir = env::current_dir().unwrap().join("public");
    // Note: in a real server, you'd want to handle errors and maybe allow configuration of the base directory.
    // For this demo, we just assume "./public" and create it if it doesn't exist.

    // create the directory and add a sample file if it doesn't exist (lab convenience)
    fs::create_dir_all(&base_dir).unwrap();
    let demo_file = base_dir.join("hello.txt");
    if !demo_file.exists() {
        fs::write(&demo_file, "hello from safe server\n").unwrap();
    }

    // instantiate server
    let server = Server::http("0.0.0.0:8000").unwrap();
    println!("Safe server on http://localhost:8000");
    println!("Serving only from: {}", base_dir.display());

    // process requests
    for request in server.incoming_requests() {
        let url = request.url().to_string();

        // use safe_path to validate the request path
        let Some(file_path) = safe_path(&base_dir, &url) else {
            // if unsafe, return 403 Forbidden
            let resp = text_response(StatusCode(403), "Forbidden\n");
            request.respond(resp).unwrap();
            continue;
        };

        // read file as bytes (not string) so binary files are possible later
        let data = match fs::read(&file_path) {
            Ok(bytes) => bytes,
            Err(_) => {
                let resp = text_response(StatusCode(404), "Not Found\n");
                request.respond(resp).unwrap();
                continue;
            }
        };

        // respond with OK and raw bytes
        let resp = Response::from_data(data).with_status_code(StatusCode(200));
        request.respond(resp).unwrap();
    }
}