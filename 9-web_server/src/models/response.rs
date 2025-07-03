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
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join("\r\n");

        // Construct response string
        if self.body.is_some() {
            format!(
                "{} {} {}\r\n{}\r\n\r\n{}",
                self.protocol,
                self.status_code.unwrap(),
                self.description.unwrap(),
                headers,
                self.body.unwrap_or("".to_owned())
            )
        } else {
            format!(
                "{} {} {}\r\n{}\r\n",
                self.protocol,
                self.status_code.unwrap(),
                self.description.unwrap(),
                headers,
            )
        }
    }

    /// Sets the `status_code` and `description` fields
    ///
    /// The `code` is the HTTP status code used to set the two fields
    pub fn set_status(&mut self, code: usize) {
        self.status_code = match code {
            200 => {
                self.description = Some("OK".to_owned());
                Some(200)
            }
            201 => {
                self.description = Some("Created".to_owned());
                Some(201)
            }
            202 => {
                self.description = Some("Accepted".to_owned());
                Some(202)
            }
            204 => {
                self.description = Some("No Content".to_owned());
                Some(204)
            }
            303 => {
                self.description = Some("See Other".to_owned());
                Some(303)
            }
            400 => {
                self.description = Some("Bad Request".to_owned());
                Some(400)
            }
            403 => {
                self.description = Some("Forbidden".to_owned());
                Some(403)
            }
            404 => {
                self.description = Some("Not Found".to_owned());
                Some(404)
            }
            500 => {
                self.description = Some("Internal Server Error".to_owned());
                Some(500)
            }
            _ => None,
        }
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Loads the contents to be sent to client into the `body` field
    ///
    /// The `contents` is the String to be sent to the client
    pub fn set_body(&mut self, contents: Option<String>) {
        self.body = contents;
    }

    /// Adds a header to the calling Response
    ///
    /// The `header` is a tuple containing the key-value pair to be added
    pub fn add_header(&mut self, header: (String, String)) {
        let (key, value) = header;
        self.headers.insert(key, value);
    }

    /// Sets the protocol field of the calling Response
    ///
    /// The `proto` is the protocol to be set. The default is `HTTP/1.1`
    pub fn set_protocol(&mut self, proto: String) {
        self.protocol = proto;
    }
}
