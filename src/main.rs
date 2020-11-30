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

    let (status_line, filename, message_body) = match buffer {
        b if b.starts_with(b"GET / HTTP") => {
            ("HTTP/1.1 200 OK\r\n\r\n", Some("src/index.html"), HTML)
        }
        b if b.starts_with(b"GET /value HTTP") => {
            ("HTTP/1.1 200 OK\r\n\r\n", None, "{ \"value\": \"TODO\" }")
        }
        _ => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", None, ""),
    };

    #[cfg(feature = "serve-files")]
    let contents = fs::read_to_string(filename).unwrap();
    #[cfg(not(feature = "serve-files"))]
    let contents = message_body;

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
