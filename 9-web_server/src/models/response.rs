use std::collections::HashMap;

pub struct Response {
    protocol: String,
    status_code: Option<usize>,
    description: Option<String>,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            protocol: String::from("HTTP/1.1"),
            status_code: None,
            description: None,
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl Response {
    /// Consumes calling Response and returns its data as a `String` in HTTP response format
    pub fn stringify(self) -> String {
        // Format headers
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");

        // Construct response string
        format!(
            "{} {} {}\r\n{}\r\n\r\n{}",
            self.protocol,
            self.status_code.unwrap_or(0),
            self.description.unwrap_or("".to_string()),
            headers,
            self.body.unwrap_or("".to_string())
        )
    }

    /// Sets the `status_code` and `description` fields
    ///
    /// The `code` is the HTTP status code used to set the two fields
    pub fn set_status(&mut self, code: usize) {
        self.status_code = match code {
            200 => {
                self.description = Some("OK".to_string());
                Some(200)
            }
            201 => {
                self.description = Some("Created".to_string());
                Some(201)
            }
            202 => {
                self.description = Some("Accepted".to_string());
                Some(202)
            }
            204 => {
                self.description = Some("No Content".to_string());
                Some(204)
            }
            400 => {
                self.description = Some("Bad Request".to_string());
                Some(400)
            }
            403 => {
                self.description = Some("Forbidden".to_string());
                Some(403)
            }
            404 => {
                self.description = Some("Not Found".to_string());
                Some(404)
            }
            500 => {
                self.description = Some("Internal Server Error".to_string());
                Some(500)
            }
            _ => None,
        }
    }

    /// Loads the contents to be sent to client into the `body` field
    ///
    /// The `contents` is the String to be sent to the client
    pub fn set_body(&mut self, _contents: String) {
        todo!()
    }
}
