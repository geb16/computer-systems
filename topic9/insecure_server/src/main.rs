use tiny_http::{Server, Response};
use std::fs;
// first we need a  function to extract the `file=`query value from the URL, 
// and then we can read the file and return its contents as the response body.
// exmaple URL: /read?file=../../etc/passwd
fn get_file_param(url: &str) -> Option<String> {
   let (_, query) = url.split_once('?')?;
    for pair in query.split('&') { //'&' is used to separate multiple query parameters, but we only care about the one with key "file"
        let (k, v) = pair.split_once('=')?;
        if k == "file" {
            return Some(v.to_string());
            // What is Some?
            // In Rust, `Option<T>` is an enum that represents a value that can either be present (Some) or absent (None).
            // `Some(T)` indicates that a value of type `T` is present, while `None` indicates that there is no value. 
            // In this context, `Some(v.to_string())` returns an `Option<String>` that contains the value of `v` converted to a String.
        }
    }
    None
}

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap(); 
    println!("Listening on http://localhost:8000");
    println!("Try: /read?file=hello.txt");
    // hello.txt should be in the same directory as the server binary, and it should contain some text for testing.
    // what if hello.txt is in a different directory? can we read it? yes, we can use relative path like /read?file=../hello.txt
    // but it reading http://localhost:8000/read?file=/etc/passwd" will also work, and it will read the contents of /etc/passwd file on the server, which is a common file that contains user account information on Unix-like systems. This is a classic example of a path traversal vulnerability, where an attacker can manipulate the file path to access sensitive files outside of the intended directory.

    for request in server.incoming_requests() {
        
        let url = request.url().to_string();
        println!("Incoming URL: {}", url);
        
        // only handle /read?... requests, otherwise return 404
        if !url.starts_with("/read?") {
            request
                .respond(Response::from_string("Use /read?file=<path>\n"))
                .unwrap();
            continue;
        }
        // 🚨 VULNERABLE: we trust user-controlled file path directly
        let Some(user_path) = get_file_param(&url) else {
            // Some is a variant of the Option enum that indicates the presence of a value.
            // In this context, `get_file_param(&url)` returns an `Option<String>`. 
            // If the function successfully extracts the file parameter from the URL, 
            // it will return `Some(String)`, where the `String` contains the value of the file parameter. 
            // If it fails to extract the parameter (e.g., if the URL does not contain a valid file parameter), 
            // it will return `None`.
            request
                .respond(Response::from_string("Missing file parameter\n"))
                .unwrap();
            continue;
        };
        let body = match fs::read_to_string(&user_path) {
            Ok(content) => content,
            Err(e) => format!("Error reading file '{}': {}\n", user_path, e),
        };
        request.respond(Response::from_string(body)).unwrap();
    }
}