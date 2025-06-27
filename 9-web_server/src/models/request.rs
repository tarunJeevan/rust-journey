use std::{collections::HashMap, path::PathBuf};

use super::http::*;

pub struct Request {
    protocol: String,
    method: HttpMethod,
    resource: PathBuf,
    queries: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Default for Request {
    fn default() -> Self {
        Request {
            protocol: String::from("HTTP/1.1"),
            method: HttpMethod::None,
            resource: PathBuf::new(),
            queries: HashMap::new(),
            headers: HashMap::new(),
            body: vec![],
        }
    }
}

impl Request {
    /// Parses a given status line into the `method`, `resource`, and `protocol` fields
    ///
    /// The `status_line` is a String containing a request's status line
    pub fn parse_status_line(&mut self, status_line: String) {
        let mut chunks = status_line.split(" ");

        // Set method
        self.method = match chunks.next() {
            Some(x) => match x.to_uppercase().as_str() {
                "GET" => HttpMethod::Get,
                "POST" => HttpMethod::Post,
                "PUT" => HttpMethod::Put,
                "DELETE" => HttpMethod::Delete,
                "OPTIONS" => HttpMethod::Options,
                _ => HttpMethod::None,
            },
            None => HttpMethod::None,
        };

        // If request method is GET, parse resource for queries
        if let HttpMethod::Get = self.method {
            let (path, query_string) = chunks.next().unwrap().split_once("?").unwrap();

            // Set resource
            self.resource = if path == "/" {
                PathBuf::from("public/index.html")
            } else {
                PathBuf::from(format!("public{}", path))
            };

            // Set queries
            let queries_list = query_string.split("&");
            for query in queries_list {
                let (key, value) = query.split_once("=").unwrap();
                self.queries.insert(key.to_string(), value.to_string());
            }
        }

        // Set protocol
        self.protocol = chunks.next().unwrap().to_string();
    }

    /// Processes and appends a given header into the headers HashMap
    ///
    /// The `line` is a String line from the headers section from a BufReader
    pub fn append_header(&mut self, line: &str) {
        if let Some((key, value)) = line.split_once(": ") {
            self.headers.insert(key.to_string(), value.to_string());
        };
    }

    /// Sets the body field of the Request
    ///
    /// The `contents` is an array slice containing all the bytes in the request body
    pub fn set_body(&mut self, contents: &[u8]) {
        self.body = contents.to_vec();
    }

    /// Returns a reference to the HttpMethod of the Request
    pub fn get_method(&self) -> &HttpMethod {
        &self.method
    }

    /// Returns a reference to the target resource of the Request
    pub fn get_resource(&self) -> &PathBuf {
        &self.resource
    }

    /// Returns a reference to the queries, if any, of the Request
    pub fn get_queries(&self) -> &HashMap<String, String> {
        &self.queries
    }

    /// Returns a reference to the headers of the Request
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns a reference to the body, if any, of the Request
    pub fn get_body(&self) -> &[u8] {
        &self.body
    }
}
