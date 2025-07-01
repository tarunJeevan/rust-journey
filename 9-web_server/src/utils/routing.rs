use std::{fs, path::Path, time::SystemTime};

use httpdate::fmt_http_date;

use crate::models::{HttpMethod, Request, Response};

/// Handles routing based on HTTP method and requested path
///
/// The `method` is the HTTP request method, `path` is the requested path, and `body` is the request body (may be empty)
pub fn route(req: Request) -> Response {
    // Check if request path is valid
    if !req.get_resource().is_file() {
        return handle_bad_request("Requested resource must be a file");
    }

    match req.get_method() {
        HttpMethod::Options => {
            // Return allowed HTTP request methods
            options()
            // TODO: Test for queries in options()
        }
        HttpMethod::Get => {
            // Return requested resource/data if it exists or return error page
            get(req)
            // TODO: Test for queries in get()
        }
        HttpMethod::Post => {
            post(req)
            // TODO: Test for queries in post()
        }
        HttpMethod::Put => {
            // Create new resource or modify existing resource SAFELY (Idempotent)
            put(req)
            // TODO: Test for queries in put()
        }
        HttpMethod::Delete => {
            // Delete a resource SAFELY (Idempotent)
            delete(req)
            // TODO: Test for queries in delete()
        }
        HttpMethod::None => invalid_request(),
    }
}

/// Handles requests with unsupported HTTP methods
///
/// Returns a `Response` containing containing the file path of the 405 error page
fn invalid_request() -> Response {
    // Initialize response
    let mut res = Response::default();

    // Set Date header
    res.add_header(set_date_header());

    // Set status line
    res.set_status(405);

    // Get contents
    let contents = read_file("public/error/405.html");

    // Set headers
    if let Some(content) = &contents {
        res.add_header(("Content-Type".to_owned(), "text/html".to_owned()));
        res.add_header(("Content-Length".to_owned(), content.len().to_string()));
    }

    // Set body
    res.set_body(contents);

    // Return response
    res
}

/// Handles bad requests
///
/// The `message` is the error message to be sent
///
/// Returns a `Response` containing an informational error message in JSON
fn handle_bad_request(message: &str) -> Response {
    // Initialize response
    let mut res = Response::default();

    // Set status line
    res.set_status(400);

    // Generate response body
    let contents = format!("{{ 'error': 'Bad request', 'message': {message} }}");

    // Set headers
    res.add_header(set_date_header());
    res.add_header(("Content-Type".to_owned(), "application/json".to_owned()));
    res.add_header(("Content-Length".to_owned(), contents.len().to_string()));

    // Set body
    res.set_body(Some(contents));

    res
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
/// The `req` is the Request struct containing request data
///
/// Returns a `Response` containing the file path of the requested resource
fn get(req: Request) -> Response {
    // Initialize response
    let mut res = Response::default();

    // Set Date header
    res.add_header(set_date_header());

    // Extract path from request
    let path = req.get_resource();

    // Return requested resource/data if it exists or return error page
    if path.exists() {
        // Get file contents
        let contents = read_file(path.to_str().unwrap_or("error"));

        // Set status line
        res.set_status(200);

        // Set headers
        if let Some(content) = &contents {
            res.add_header(get_content_type(path));
            res.add_header(("Content-Length".to_owned(), content.len().to_string()));
        }

        // Set body
        res.set_body(contents);
    } else {
        let contents = read_file("public/error/404.html");
        // Set status line
        res.set_status(404);

        // Set headers
        if let Some(content) = &contents {
            res.add_header(get_content_type(Path::new("public/error/404.html")));
            res.add_header(("Content-Length".to_owned(), content.len().to_string()));
        }

        // Set body
        res.set_body(contents);
    }

    // Return response
    res
}

/// Handles `POST` requests
///
/// The `req` is the Request struct containing request data
///
/// Returns a `Response` redirecting to success file path
fn post(req: Request) -> Response {
    // Initialize response
    let mut res = Response::default();

    // Set Date header
    res.add_header(set_date_header());

    // Process request body
    if let Some(value) = req.get_headers().get("content-type") {
        match value.as_str() {
            "application/x-www-form-urlencoded" => {
                let body_str = String::from_utf8_lossy(req.get_body());
                let params: Vec<(&str, &str)> = body_str
                    .split("&")
                    .filter_map(|pair| {
                        let mut kv = pair.splitn(2, "=");
                        Some((kv.next()?, kv.next()?))
                    })
                    .collect();
                // NOTE: Do something with collected params

                // Set response body
                let res_body = params
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<String>>()
                    .join(", ");

                // Write processed data to file
                if let Some(error_file) =
                    write_to_file(Path::new("public/post-success.txt"), res_body.as_bytes())
                {
                    // Handle file writing error
                    // Set status line
                    res.set_status(500);

                    // Set headers
                    res.add_header(get_content_type(Path::new("public/error/500.html")));
                    res.add_header(("Content-Length".to_owned(), error_file.len().to_string()));

                    // Set body
                    res.set_body(Some(error_file));
                }

                // Redirect on success
                // Set status line
                res.set_status(303);

                // Set headers
                let redirect = Path::new("public/post-success.txt");
                res.add_header(get_content_type(redirect));
                res.add_header((
                    "Location".to_owned(),
                    redirect.to_str().map(|val| val.to_owned()).unwrap(),
                ));
            }
            "multiplart/form-data" => {
                // TODO: Used for file uploads. Implement later
                todo!("miltipart/form-data not implemented yet")
            }
            "text/plain" | "application/octet-stream" => {
                // Write data to file
                if let Some(error_file) =
                    write_to_file(Path::new("public/post-success.txt"), req.get_body())
                {
                    // Handle file writing error
                    // Set status line
                    res.set_status(500);

                    // Set headers
                    res.add_header(get_content_type(Path::new("public/error/500.html")));
                    res.add_header(("Content-Length".to_owned(), error_file.len().to_string()));

                    // Set body
                    res.set_body(Some(error_file));
                }

                // Redirect on success
                // Set status line
                res.set_status(303);

                // Set headers
                let redirect = Path::new("public/post-success.txt");
                res.add_header(get_content_type(redirect));
                res.add_header((
                    "Location".to_owned(),
                    redirect.to_str().map(|val| val.to_owned()).unwrap(),
                ));
            }
            _ => {
                // NOTE: Content-Type can be anything if POST is not sent by a form
                todo!("Unimplemented Content-Type on POST")
            }
        }
    }

    res
}

/// Handles `PUT` requests
///
/// The `req` is the Request struct containing request data
///
/// Returns a `Response` containing the file path of created/modified resource
fn put(req: Request) -> Response {
    // Initialize response
    let mut res = Response::default();

    // Set Date header
    res.add_header(set_date_header());

    // Extract path and body from request
    let path = req.get_resource();
    let body = req.get_body();

    // Check if resource exists
    if path.exists() {
        // File exists so modify it. Handle error if it occurs
        if let Some(error_file) = write_to_file(path, body) {
            // Set status line
            res.set_status(500);

            // Set headers
            res.add_header(get_content_type(Path::new("public/error/500.html")));
            res.add_header(("Content-Length".to_owned(), error_file.len().to_string()));

            // Set body
            res.set_body(Some(error_file));
        }

        // Successfully modified
        let contents = read_file(path.to_str().unwrap_or("error"));

        // Set status line
        res.set_status(200);

        // Set headers
        if let Some(content) = &contents {
            res.add_header(get_content_type(path));
            res.add_header(("Content-Length".to_owned(), content.len().to_string()));
        }

        // Set body
        res.set_body(contents);
    }
    // File doesn't exist so create it
    else {
        // Write to file and handle error if it occurs
        if let Some(error_file) = write_to_file(path, body) {
            // Set status line
            res.set_status(500);

            // Set headers
            res.add_header(get_content_type(Path::new("public/error/500.html")));
            res.add_header(("Content-Length".to_owned(), error_file.len().to_string()));

            // Set body
            res.set_body(Some(error_file));
        }

        // Successfully created
        let contents = read_file(path.to_str().unwrap_or("error"));

        // Set status line
        res.set_status(201);

        // Set headers
        if let Some(content) = &contents {
            res.add_header(get_content_type(path));
            res.add_header(("Content-Length".to_owned(), content.len().to_string()));
        }

        // Set body
        res.set_body(contents);
    }

    // Return response
    res
}

/// Handles `DELETE` requests
///
/// The `req` is the Request struct containing request data
///
/// Returns a `Response` containing the redirection file path (empty `String` if successful)
fn delete(req: Request) -> Response {
    // Initialize response
    let mut res = Response::default();

    // Set Date header
    res.add_header(set_date_header());

    // Extract path from request
    let path = req.get_resource();

    // Check if file-to-delete exists
    if path.exists() {
        fs::remove_file(path).unwrap_or_else(|e| {
            eprintln!("File deletion error: {e}");

            // Send error page
            let contents = read_file("public/error/500.html");

            // Set status line
            res.set_status(500);

            // Set headers
            if let Some(content) = &contents {
                res.add_header(get_content_type(Path::new("public/error/500.html")));
                res.add_header(("Content-Length".to_owned(), content.len().to_string()));
            }

            // Set body
            res.set_body(contents);
        });
        // File successfully deleted
        res.set_status(204);
        // NOTE: In calling code check path and refresh page on successful deletion
    } else {
        // File-to-delete not found
        let contents = read_file("public/error/404.html");

        // Set status line
        res.set_status(404);

        // Set headers
        if let Some(content) = &contents {
            res.add_header(get_content_type(Path::new("public/error/404.html")));
            res.add_header(("Content-Length".to_owned(), content.len().to_string()));
        }

        // Set body
        res.set_body(contents);
    }

    // Return response
    res
}

/// Determines Date header
///
/// Returns a tuple of two Strings containing the header name and computed value
fn set_date_header() -> (String, String) {
    let now = SystemTime::now();
    ("Date".to_owned(), fmt_http_date(now))
}

/// Determines the correct `Content-Type` for a given file path
///
/// The `path` is the file path to be analyzed
///
/// Returns a tuple of two Strings containing the header name and computed value
fn get_content_type(path: &Path) -> (String, String) {
    let content_type = match path.extension().and_then(|p| p.to_str()).unwrap() {
        "html" => "text/html",
        "pdf" => "application/pdf",
        "json" => "application/json",
        "js" | "mjs" => "text/javascript",
        "css" => "text/css",
        "jpeg" | "jpg" => "image/jpeg",
        "png" => "image/png",
        "txt" => "text/plain",
        "md" => "text/markdown",
        _ => "application/octet-stream",
    }
    .to_owned();

    ("Content-Type".to_owned(), content_type)
}

/// Read and return file contents
///
/// The `file_path` is the file path to read from
///
/// Returns an Option containing the file contents or None if an error occurs
fn read_file(file_path: &str) -> Option<String> {
    match fs::read_to_string(file_path) {
        Ok(contents) => Some(contents),
        Err(err) => {
            eprintln!("Error reading {file_path}: {err}");
            None
        }
    }
}

/// Write to a file, overwriting an existing file or creating a new one
///
/// The `file_path` is the target file path and `contents` is the payload (in bytes)
///
/// Returns an Option containing the contents of an error file or None on success
fn write_to_file(file_path: &Path, contents: &[u8]) -> Option<String> {
    match fs::write(file_path, contents) {
        Ok(_) => None,
        Err(err) => {
            eprintln!("Error writing to file: {err}");
            read_file("public/error/500.html")
        }
    }
}
