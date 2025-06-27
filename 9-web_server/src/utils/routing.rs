use std::{fs, path::Path};

use crate::models::{HttpMethod, Request, Response};

/// Handles routing based on HTTP method and requested path
///
/// The `method` is the HTTP request method, `path` is the requested path, and `body` is the request body (may be empty)
pub fn route(req: Request) -> Response {
    match req.get_method() {
        HttpMethod::Options => {
            // Return allowed HTTP request methods
            options()
        }
        HttpMethod::Get => {
            // Return requested resource/data if it exists or return error page
            get(req.get_resource())
        }
        HttpMethod::Post => {
            // TODO: Handle submitted data
            post(req.get_resource(), req.get_body())
        }
        HttpMethod::Put => {
            // Create new resource or modify existing resource SAFELY (Idempotent)
            put(req.get_resource(), req.get_body())
        }
        HttpMethod::Delete => {
            // Delete a resource SAFELY (Idempotent)
            delete(req.get_resource())
        }
        HttpMethod::None => {
            // Return invalid request method
            // (
            //     "HTTP/1.1 405 NOT ALLOWED",
            //     "public/error/405.html".to_string(),
            //     String::new(),
            // )
            Response::default()
        }
    }
}

/// Handles `OPTIONS` request
///
/// Returns a `Response` containing `Allow` header detailing permitted HTTP request methods
fn options() -> Response {
    // Request {
    //     status_line: "HTTP/1.1 204 No Content".to_string(),
    //     headers: vec![(
    //         "Allow".to_string(),
    //         "GET, POST, PUT, DELETE, OPTIONS".to_string(),
    //     )],
    //     body: String::new(),
    // }

    Response::default()
}

/// Handles `GET` requests
///
/// The `path` is the path to the requested resource
///
/// Returns a `Response` containing the file path of the requested resource
fn get(path: &Path) -> Response {
    // Return requested resource/data if it exists or return error page
    if path.exists() {
        let _body = fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading file to string: {err}");
            String::new()
        });
        // Request {
        //     status_line: "HTTP/1.1 200 OK".to_string(),
        //     headers: vec![("Content-Length".to_string(), body.len().to_string())],
        //     body,
        // }
        Response::default()
    } else {
        let _body = fs::read_to_string("public/error/404.html").unwrap_or_else(|err| {
            eprintln!("Error reading error file to string: {err}");
            String::new()
        });
        // Request {
        //     status_line: "HTTP/1.1 404 Not Found".to_string(),
        //     headers: vec![("Content-Length".to_string(), body.len().to_string())],
        //     body,
        // }
        Response::default()
    }
}

/// Handles `POST` requests
///
/// The `path` is the ? and `body` is the submitted data
///
/// Returns a `Response` containing ?
fn post(_path: &Path, _body: &[u8]) -> Response {
    todo!()
}

/// Handles `PUT` requests
///
/// The `path` is the resource to be created/modified and `body` is the submitted data
///
/// Returns a `Response` containing the file path of created/modified resource
fn put(path: &Path, body: &[u8]) -> Response {
    let mut status_line = String::new();
    let mut contents = String::new();

    // Check if resource exists
    if path.exists() {
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
            eprintln!("Error reading error file to string: {err}");
            String::new()
        });
    }

    // Construct and return response
    // Request {
    //     status_line,
    //     headers: vec![("Content-Length".to_string(), contents.len().to_string())],
    //     body: contents,
    // }
    Response::default()
}

/// Handles `DELETE` requests
///
/// The `path` is the resource to be deleted
///
/// Returns a `Response` containing the redirection file path (empty `String` if successful)
fn delete(path: &Path) -> Response {
    let mut status_line = String::new();
    let mut body = String::new();

    // Check if file-to-delete exists
    if path.exists() {
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
    // Request {
    //     status_line,
    //     headers: vec![("Content-Length".to_string(), body.len().to_string())],
    //     body,
    // }
    Response::default()
}
