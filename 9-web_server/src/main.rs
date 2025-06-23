use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
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

    // Construct response based on request
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
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
