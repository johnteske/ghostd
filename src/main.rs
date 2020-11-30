#[cfg(feature = "serve-files")]
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

//use std::sync::Arc;
//use std::sync::RwLock;
//use std::time::{Duration, Instant};

#[cfg(not(feature = "serve-files"))]
include!(concat!(env!("OUT_DIR"), "/html.rs"));

//const TIMEOUT: Duration = Duration::from_secs(120);

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4321").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/index.html")
    } else if buffer.starts_with(b"GET /value") {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/i")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    #[cfg(feature = "serve-files")]
    let contents = fs::read_to_string(filename).unwrap();
    #[cfg(not(feature = "serve-files"))]
    let contents = filename;

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
