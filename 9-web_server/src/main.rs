use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
};

use web_server::{
    models::ThreadPool,
    utils::{parse_request, route},
};

fn main() {
    // Create TCP listener bound to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // TODO: Handle possible error case
    let pool = ThreadPool::new(50);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/// Handles each request from client
///
/// The `stream` is the TcpStream containing the HTTP request
fn handle_connection(mut stream: TcpStream) {
    // Get all lines from stream
    let mut buf_reader = BufReader::new(&stream);
    let req = parse_request(&mut buf_reader);

    // Construct response based on request
    let res = route(req);

    // Send response
    stream
        .write_all(res.stringify().as_bytes())
        .unwrap_or_else(|err| eprintln!("Error sending response: {err}"));
    // NOTE: Flush stream after?
}
