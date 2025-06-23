use std::{
    fs,
    io::{BufRead, BufReader, Write},
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
    let buf_reader = BufReader::new(&stream);
    // Get request line
    let request_line = buf_reader.lines().next().unwrap().unwrap_or_else(|err| {
        eprintln!("Error reading request line: {err}");
        String::new()
    });

    // Parse request line into method, path, and version
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let _version = parts.next().unwrap(); // Unnecessary

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
            // TODO: Return allowed HTTP request methods
            todo!()
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
            // TODO: Handle creating new resource or modifying existing resource SAFELY (Idempotent)
            todo!()
        }
        "DELETE" => {
            // TODO: Handle deleting a resource SAFELY (Idempotent)
            todo!()
        }
        _ => {
            // TODO: Return invalid request method
            todo!()
        }
    }
}
