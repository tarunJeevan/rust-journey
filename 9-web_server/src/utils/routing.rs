use std::{fs, path::Path, time::SystemTime};

use httpdate::fmt_http_date;

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
    // Construct response
    let mut res = Response::default();
    res.set_status(204);
    res.add_header((
        "Allow".to_owned(),
        "GET, POST, PUT, DELETE, OPTIONS".to_owned(),
    ));

    // Return response
    res
}

/// Handles `GET` requests
///
/// The `path` is the path to the requested resource
///
/// Returns a `Response` containing the file path of the requested resource
fn get(path: &Path) -> Response {
    // Initialize response
    let mut res = Response::default();

    // Set Date header
    res.add_header(set_date_header());

    // Return requested resource/data if it exists or return error page
    if path.exists() {
        let contents = fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading file to string: {err}");
            String::new()
        });
        // Set status line
        res.set_status(200);

        // Set headers
        res.add_header(("Content-Length".to_owned(), contents.len().to_string()));
        // TODO: Add Content-Type?

        // Set body
        res.set_body(contents);
    } else {
        let contents = fs::read_to_string("public/error/404.html").unwrap_or_else(|err| {
            eprintln!("Error reading error file to string: {err}");
            String::new()
        });
        // Set status line
        res.set_status(404);

        // Set headers
        res.add_header(("Content-Length".to_owned(), contents.len().to_string()));
        // TODO: Add Content-Type?

        // Set body
        res.set_body(contents);
    }
    for (key, value) in res.get_headers() {
        println!("{}: {}", key, value);
    }

    // Return response
    res
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
    // Initialize response
    let mut res = Response::default();

    // Set Date header
    res.add_header(set_date_header());

    // Check if resource exists
    if path.exists() {
        // File exists so modify it
        fs::write(path, body).unwrap_or_else(|e| {
            // Internal error while modifying file
            eprintln!("Error writing file: {e}");
            let contents = fs::read_to_string("public/error/500.html").unwrap_or_else(|err| {
                eprintln!("Error reading error file to string: {err}");
                String::new()
            });

            // Set status line
            res.set_status(500);

            // Set headers
            res.add_header(("Content-Length".to_owned(), contents.len().to_string()));
            // TODO: Add Content-Type?

            // Set body
            res.set_body(contents);
        });
        // Successfully modified
        let contents = fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading file to string: {err}");
            String::new()
        });

        // Set status line
        res.set_status(200);

        // Set headers
        res.add_header(("Content-Length".to_owned(), contents.len().to_string()));
        // TODO: Add Content-Type?

        // Set body
        res.set_body(contents);
    }
    // File doesn't exist so create it
    else {
        fs::write(path, body).unwrap_or_else(|e| {
            // Internal error while creating file
            eprintln!("Error writing file: {e}");

            let contents = fs::read_to_string("public/error/500.html").unwrap_or_else(|err| {
                eprintln!("Error reading error file to string: {err}");
                String::new()
            });

            // Set status line
            res.set_status(500);

            // Set headers
            res.add_header(("Content-Length".to_owned(), contents.len().to_string()));
            // TODO: Add Content-Type?

            // Set body
            res.set_body(contents);
        });
        // Successfully created
        let contents = fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading error file to string: {err}");
            String::new()
        });

        // Set status line
        res.set_status(201);

        // Set headers
        res.add_header(("Content-Length".to_owned(), contents.len().to_string()));
        // TODO: Add Content-Type?

        // Set body
        res.set_body(contents);
    }

    // Return response
    res
}

/// Handles `DELETE` requests
///
/// The `path` is the resource to be deleted
///
/// Returns a `Response` containing the redirection file path (empty `String` if successful)
fn delete(path: &Path) -> Response {
    // Initialize response
    let mut res = Response::default();

    // Check if file-to-delete exists
    if path.exists() {
        fs::remove_file(path).unwrap_or_else(|e| {
            eprintln!("File deletion error: {e}");

            // Send error page
            let contents = fs::read_to_string("public/error/500.html").unwrap_or_else(|err| {
                eprintln!("Error reading error file to string: {err}");
                String::new()
            });

            // Set status line
            res.set_status(500);

            // Set headers
            res.add_header(("Content-Length".to_owned(), contents.len().to_string()));
            // TODO: Add Content-Type?

            // Set body
            res.set_body(contents);
        });
        // File successfully deleted
        res.set_status(204);
        // NOTE: In calling code check path and refresh page on successful deletion
    } else {
        // File-to-delete not found
        let contents = fs::read_to_string("public/error/404.html").unwrap_or_else(|err| {
            eprintln!("Error reading error file to string: {err}");
            String::new()
        });

        // Set status line
        res.set_status(404);

        // Set headers
        res.add_header(("Content-Length".to_owned(), contents.len().to_string()));
        // TODO: Add Content-Type?

        // Set body
        res.set_body(contents);
    }

    // Return response
    res
}

// Helper function to set Date header for Response
fn set_date_header() -> (String, String) {
    let now = SystemTime::now();
    ("Date".to_owned(), fmt_http_date(now))
}
