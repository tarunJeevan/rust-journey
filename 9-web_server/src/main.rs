use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

use web_server::ThreadPool;

struct Response {
    status_line: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    fn stringify(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");
        format!("{}\r\n{}\r\n\r\n{}", self.status_line, headers, self.body)
    }
}

fn main() {
    // Create TCP listener bound to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // TODO: Handle possible error case
    let pool = ThreadPool::new(5);

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
    let (request_line, headers, body) = parse_request(&mut buf_reader);

    // Parse request line into method, path, and version
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let _version = parts.next().unwrap(); // Version is unnecessary

    // Construct response based on request
    let (status_line, filename, response_body) = route(method, path, &body);

    let contents = if response_body.is_empty() {
        if filename.is_empty() {
            String::new()
        } else {
            fs::read_to_string(filename).unwrap_or_else(|err| {
                eprintln!("Error reading file to string: {err}");
                String::new()
            })
        }
    } else {
        response_body
    };
    let content_len = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");

    // Send response
    stream
        .write_all(response.as_bytes())
        .unwrap_or_else(|err| eprintln!("Error sending response: {err}"));
}

/// Parses HTTP request from client
///
/// The `buf_reader` is a buffered reader containing the `TcpStream` for easier processing
///
/// Returns a tuple containing the start line, headers, and body from the processed request
fn parse_request(
    buf_reader: &mut BufReader<&TcpStream>,
) -> (String, HashMap<String, String>, Vec<u8>) {
    let mut request_line = String::new();

    // Read the request line
    buf_reader.read_line(&mut request_line).unwrap();

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
    }

    // Read request body if present
    let mut body = Vec::new();
    if let Some(cl) = headers.get("Content-Length") {
        if let Ok(content_length) = cl.parse::<usize>() {
            let mut body_buf = vec![0; content_length];
            buf_reader.read_exact(&mut body_buf).unwrap_or(());
            body = body_buf;
        }
    }

    // Return tuple containing parsed info
    (request_line, headers, body)
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
                    "public/error/404.html".to_string(),
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
                    "public/error/500.html".to_string(),
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
                        "public/error/500.html".to_string(),
                        String::new(),
                    )
                } else {
                    // Delete successful
                    (
                        "HTTP/1.1 200 OK",
                        "public/delete-success.html".to_string(),
                        String::new(),
                    )
                }
            } else {
                // File-to-delete not found
                (
                    "HTTP/1.1 404 NOT FOUND",
                    "public/error/404.html".to_string(),
                    String::new(),
                )
            }
        }
        _ => {
            // Return invalid request method
            (
                "HTTP/1.1 405 NOT ALLOWED",
                "public/error/405.html".to_string(),
                String::new(),
            )
        }
    }
    // NOTE: Create Response struct to be returned by route()?
}

/// Handles `OPTIONS` request
///
/// Returns response start line and header detailing permitted HTTP request methods
fn options() -> Response {
    Response {
        status_line: "HTTP/1.1 204 No Content".to_string(),
        headers: vec![(
            "Allow".to_string(),
            "GET, POST, PUT, DELETE, OPTIONS".to_string(),
        )],
        body: String::new(),
    }
}

/// Handles `GET` requests
///
/// The `path` is the path to the requested resource
///
/// Returns a tuple containing response start line and file path of the requested resource
fn get(path: &str) -> Response {
    // Return requested resource/data if it exists or return error page
    if Path::new(&path).exists() {
        let body = fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading file to string: {err}");
            String::new()
        });
        Response {
            status_line: "HTTP/1.1 200 OK".to_string(),
            headers: vec![("Content-Length".to_string(), body.len().to_string())],
            body,
        }
    } else {
        let body = fs::read_to_string("public/error/404.html").unwrap_or_else(|err| {
            eprintln!("Error reading error file to string: {err}");
            String::new()
        });
        Response {
            status_line: "HTTP/1.1 404 Not Found".to_string(),
            headers: vec![("Content-Length".to_string(), body.len().to_string())],
            body,
        }
    }
}

/// Handles `POST` requests
///
/// The `path` is the ? and `body` is the submitted data
///
/// Returns a tuple containing response start line (and ?)
fn post(path: &str, body: &[u8]) -> &'static str {
    // NOTE: Determine return type
    todo!()
}

/// Handles `PUT` requests
///
/// The `path` is the resource to be created/modified and `body` is the submitted data
///
/// Returns a tuple containing response start line and file path of created/modified resource
fn put<'a>(path: &'a str, body: &[u8]) -> Response {
    let mut status_line = String::new();
    let mut contents = String::new();

    // Check if resource exists
    if Path::new(&path).exists() {
        // File exists so modify it
        fs::write(path, body).unwrap_or_else(|e| {
            // Internal error while modifying file
            eprintln!("Error writing file: {e}");
            status_line = "HTTP/1.1 500 Internal Server Error".to_string();
            contents = fs::read_to_string("public/error/500.html").unwrap_or_else(|err| {
                eprintln!("Error reading error file to string: {err}");
                String::new()
            });
        });
        // Successfully modified
        status_line = "HTTP/1.1 200 OK".to_string();
        contents = fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading file to string: {err}");
            String::new()
        });
    }
    // File doesn't exist so create it
    else {
        fs::write(path, body).unwrap_or_else(|e| {
            // Internal error while creating file
            eprintln!("Error writing file: {e}");
            status_line = "HTTP/1.1 500 Internal Server Error".to_string();
            contents = fs::read_to_string("public/error/500.html").unwrap_or_else(|err| {
                eprintln!("Error reading error file to string: {err}");
                String::new()
            });
        });
        // Successfully created
        status_line = "HTTP/1.1 201 Created".to_string();
        contents = fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading file to string: {err}");
            String::new()
        });
    }

    // Construct and return response
    Response {
        status_line,
        headers: vec![("Content-Length".to_string(), contents.len().to_string())],
        body: contents,
    }
}

/// Handles `DELETE` requests
///
/// The `path` is the resource to be deleted
///
/// Returns a tuple containing response start line and redirection file path (empty `&str` if successful)
fn delete(path: &str) -> Response {
    let mut status_line = String::new();
    let mut body = String::new();

    // Check if file-to-delete exists
    if Path::new(&path).exists() {
        fs::remove_file(path).unwrap_or_else(|e| {
            eprintln!("File deletion error: {e}");
            status_line = "HTTP/1.1 500 Internal Server Error".to_string();
            body = fs::read_to_string("public/error/500.html").unwrap_or_else(|err| {
                eprintln!("Error reading error file to string: {err}");
                String::new()
            });
        });
        status_line = "HTTP/1.1 204 No Content".to_string();
        // NOTE: In calling code check path and refresh page on successful deletion
    } else {
        // File-to-delete not found
        status_line = "HTTP/101 404 Not Found".to_string();
        body = fs::read_to_string("public/error/404.html").unwrap_or_else(|err| {
            eprintln!("Error reading error file to string: {err}");
            String::new()
        });
    }

    // Construct and return response
    Response {
        status_line,
        headers: vec![("Content-Length".to_string(), body.len().to_string())],
        body,
    }
}
