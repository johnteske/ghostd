#[cfg(feature = "serve-files")]
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// use std::sync::Arc;
// use std::sync::RwLock;
// use std::time::{Duration, Instant};

#[cfg(not(feature = "serve-files"))]
include!(concat!(env!("OUT_DIR"), "/html.rs"));

// const TIMEOUT: Duration = Duration::from_secs(120);

const OK_200: &str = "200 OK";
const NOT_FOUND_404: &str = "404 NOT FOUND";

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

    let (status, filename, message_body) = match buffer {
        b if b.starts_with(b"GET / HTTP") => (OK_200, Some("src/index.html"), HTML),
        b if b.starts_with(b"GET /value HTTP") => (OK_200, None, "{ \"value\": \"TODO\" }"),
        _ => (NOT_FOUND_404, None, ""),
    };

    #[cfg(feature = "serve-files")]
    let contents = fs::read_to_string(filename).unwrap();
    #[cfg(not(feature = "serve-files"))]
    let contents = message_body;

    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
