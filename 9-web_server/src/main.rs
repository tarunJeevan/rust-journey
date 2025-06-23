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

/// Handles each request from client
///
/// The `stream` is the TcpStream containing the HTTP request
fn handle_connection(mut stream: TcpStream) {
    // Get all lines from stream
    let mut buf_reader = BufReader::new(&stream);
    let mut lines = Vec::new();
    let mut request_line = String::new();

    // Read the request line
    buf_reader.read_line(&mut request_line).unwrap();
    lines.push(request_line.trim_end().to_string());

    // Read headers
    let mut headers = HashMap::new();
    let mut line = String::new();
    loop {
        line.clear();
        buf_reader.read_line(&mut line).unwrap();
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
        lines.push(trimmed.to_string());
    }

    // Parse request line into method, path, and version
    let mut parts = lines[0].split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let _version = parts.next().unwrap(); // Version is unnecessary

    // Read request body if present
    let mut body = Vec::new();
    if let Some(cl) = headers.get("Content-Length") {
        if let Ok(content_length) = cl.parse::<usize>() {
            let mut body_buf = vec![0; content_length];
            buf_reader.read_exact(&mut body_buf).unwrap_or(());
            body = body_buf;
        }
    }

    // Construct response based on request
    let (status_line, filename, response_body) = route(method, path, &body);
    let contents = if !response_body.is_empty() {
        response_body
    } else {
        fs::read_to_string(filename).unwrap_or_else(|err| {
            eprintln!("Error reading file to string: {err}");
            String::new()
        })
    };
    let content_len = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");

    // Send response
    stream
        .write_all(response.as_bytes())
        .unwrap_or_else(|err| eprintln!("Error sending response: {err}"));
}

/// Handles routing based on HTTP method and requested path
///
/// The `method` is the HTTP request method, `path` is the requested path, and `body` is the request body (may be empty)
fn route(method: &str, path: &str, body: &[u8]) -> (&'static str, String, String) {
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
                String::new(),
            )
        }
        "GET" => {
            // Return requested resource/data if it exists or return error page
            if Path::new(&file_path).exists() {
                ("HTTP/1.1 200 OK", file_path, String::new())
            } else {
                (
                    "HTTP/1.1 404 NOT FOUND",
                    "public/404.html".to_string(),
                    String::new(),
                )
            }
        }
        "POST" => {
            // TODO: Handle submitted data
            todo!()
        }
        "PUT" => {
            // Create new resource or modify existing resource SAFELY (Idempotent)
            if let Err(e) = fs::write(&file_path, body) {
                eprintln!("Error writing file: {e}");
                (
                    "HTTP/1.1 500 INTERNAL SERVER ERROR",
                    "public/500.html".to_string(),
                    String::new(),
                )
            } else {
                ("HTTP/1.1 201 CREATED", file_path, String::new())
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
                        String::new(),
                    )
                } else {
                    // Delete successful
                    (
                        "HTTP/1.1 200 OK",
                        "public/delete_success.html".to_string(),
                        String::new(),
                    )
                }
            } else {
                // File-to-delete not found
                (
                    "HTTP/1.1 404 NOT FOUND",
                    "public/404.html".to_string(),
                    String::new(),
                )
            }
        }
        _ => {
            // Return invalid request method
            (
                "HTTP/1.1 405 NOT ALLOWED",
                "public/405.html".to_string(),
                String::new(),
            )
        }
    }
}
