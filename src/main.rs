#[cfg(feature = "serve-files")]
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::sync::Arc;
use std::sync::RwLock;
// use std::time::{Duration, Instant};

#[cfg(not(feature = "serve-files"))]
include!(concat!(env!("OUT_DIR"), "/html.rs"));

// const TIMEOUT: Duration = Duration::from_secs(120);

const OK_200: &str = "200 OK";
const NOT_FOUND_404: &str = "404 NOT FOUND";

struct State {
    value: String,
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4321").unwrap();

    let state = Arc::new(RwLock::new(State {
        value: "UNSET".to_string(),
    }));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, state.clone());
    }
}

fn handle_connection(mut stream: TcpStream, state: Arc<RwLock<State>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status, filename, message_body): (&str, Option<&str>, String) = match buffer {
        // assets
        b if b.starts_with(b"GET / HTTP") => (OK_200, Some("src/index.html"), HTML.to_string()),
        // values
        b if b.starts_with(b"GET /value HTTP") => {
            let st = state.clone();
            let st = st.read().unwrap();
            (OK_200, None, format!("{{ \"value\": \"{}\" }}", st.value))
        }
        b if b.starts_with(b"POST /value HTTP") => {
            let s = String::from_utf8(b.to_vec()).expect("error converting request");
            let s = s.trim_end_matches("\0"); // remove NUL
            let body = s.lines().last().expect("error getting request body");

            let st = state.clone();
            let mut st = st.write().unwrap();
            *st = State {
                value: body.to_string(),
            };
            (
                OK_200,
                None,
                format!("{{ \"value\": \"{}\" }}", body.to_string()),
            )
        }
        // 404
        _ => (NOT_FOUND_404, None, "".to_string()),
    };

    #[cfg(feature = "serve-files")]
    let contents = fs::read_to_string(filename).unwrap();
    #[cfg(not(feature = "serve-files"))]
    let contents = message_body;

    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
