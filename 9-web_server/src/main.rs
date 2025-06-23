use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

use web_server::ThreadPool;

fn main() {
    // Create TCP listener bound to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // TODO: Handle possible error case
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down...");
}

fn handle_connection(mut stream: TcpStream) {
    // Get all lines from stream
    let mut buf_reader = BufReader::new(&stream);
    let lines: Vec<_> = buf_reader.by_ref().lines().map_while(Result::ok).collect();

    // Get request line
    let request_line = &lines[0];

    // Parse request line into method, path, and version
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let _version = parts.next().unwrap(); // Version is unnecessary

    // Retrieve request headers
    let mut headers = HashMap::new();
    for line in &lines[1..] {
        if line.is_empty() {
            break;
        };
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    // Construct response based on request
    let (status_line, filename) = route(method, path);
    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file to string: {err}");
        String::new()
    });
    let content_len = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");

    // Send response
    stream
        .write_all(response.as_bytes())
        .unwrap_or_else(|err| eprintln!("Error sending response: {err}"));
}

/// Handles routing based on HTTP method and requested path
///
/// The `method` is the HTTP request method and `path` is the requested path
fn route(method: &str, path: &str) -> (&'static str, String) {
    // Construct file path
    let file_path = if path == "/" {
        "public/index.html".to_string()
    } else {
        format!("public{}", path)
    };

    match method {
        "OPTIONS" => {
            // Return allowed HTTP request methods
            (
                "HTTP/1.1 204 NO CONTENT\r\nAllow: GET, POST, PUT, DELETE, OPTIONS\r\n\r\n",
                String::new(),
            )
        }
        "GET" => {
            // Return requested resource/data if it exists or return error page
            if Path::new(&file_path).exists() {
                ("HTTP/1.1 200 OK", file_path)
            } else {
                ("HTTP/1.1 404 NOT FOUND", "public/404.html".to_string())
            }
        }
        "POST" => {
            // TODO: Handle submitted data
            todo!()
        }
        "PUT" => {
            // Create new resource or modify existing resource SAFELY (Idempotent)
            let put_path = format!("public{}", path);

            if let Err(e) = fs::write(&put_path, "TODO: Get response body first") {
                eprintln!("Error writing file: {e}");
                (
                    "HTTP/1.1 500 INTERNAL SERVER ERROR",
                    "public/500.html".to_string(),
                )
            } else {
                ("HTTP/1.1 201 CREATED", put_path)
            }
        }
        "DELETE" => {
            // Delete a resource SAFELY (Idempotent)

            // Check if file-to-delete exists
            if Path::new(&file_path).exists() {
                if let Err(e) = fs::remove_file(&file_path) {
                    // Error deleting file
                    eprintln!("File deletion error: {e}");
                    (
                        "HTTP/1.1 500 INTERNAL SERVER ERROR",
                        "public/500.html".to_string(),
                    )
                } else {
                    // Delete successful
                    ("HTTP/1.1 200 OK", "public/delete_success.html".to_string())
                }
            } else {
                // File-to-delete not found
                ("HTTP/1.1 404 NOT FOUND", "public/404.html".to_string())
            }
        }
        _ => {
            // Return invalid request method
            ("HTTP/1.1 405 NOT ALLOWED", "public/405.html".to_string())
        }
    }
}
