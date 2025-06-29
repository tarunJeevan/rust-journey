use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::models::Request;

/// Parses HTTP request from client
///
/// The `buf_reader` is a buffered reader containing the `TcpStream` for easier processing
///
/// Returns a processed Request containing all parsed data
pub fn parse_request(buf_reader: &mut BufReader<&TcpStream>) -> Request {
    let mut req = Request::default();
    let mut status_line = String::new();

    // Read the request line
    buf_reader.read_line(&mut status_line).unwrap();
    req.parse_status_line(status_line);

    // Read headers
    let mut line = String::new();
    loop {
        line.clear();
        buf_reader.read_line(&mut line).unwrap();
        let trimmed = line.trim_end();

        if trimmed.is_empty() {
            break;
        }
        req.append_header(trimmed);
    }

    // Read request body if present
    if let Some(cl) = req.get_headers().get("Content-Length") {
        if let Ok(content_length) = cl.parse::<usize>() {
            let mut body_buf = vec![0; content_length];
            buf_reader.read_exact(&mut body_buf).unwrap_or(());
            // body = body_buf;
            req.set_body(&body_buf);
        }
    }

    // Return constructed Reqest
    req
}
