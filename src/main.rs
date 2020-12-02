use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;

mod state;
use state::State;

mod timeout;
use timeout::TIMEOUT;

include!(concat!(env!("OUT_DIR"), "/html.rs"));

const OK_200: &str = "200 OK";
const NOT_FOUND_404: &str = "404 NOT FOUND";

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4321").unwrap();

    let state = Arc::new(RwLock::new(State {
        value: "".to_string(),
        expires_at: Instant::now() + TIMEOUT,
    }));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, state.clone());
    }
}

fn handle_connection(mut stream: TcpStream, state: Arc<RwLock<State>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status, content_type, message_body): (&str, &str, String) = match buffer {
        // assets
        b if b.starts_with(b"GET / ") => (OK_200, "text/html", HTML.to_string()),
        // values
        b if b.starts_with(b"GET /value ") => {
            let st = state.clone();
            let st = st.read().unwrap();

            (OK_200, "application/json", format!("{{ \"value\": \"{}\" }}", st.value))
        }
        b if b.starts_with(b"POST /value ") => {
            let s = String::from_utf8(b.to_vec()).expect("error converting request");
            let s = s.trim_end_matches("\0"); // remove NUL
            let body = s.lines().last().expect("error getting request body");

            let st = state.clone();
            let mut st = st.write().unwrap();
            *st = State {
                value: body.to_string(),
                expires_at: st.expires_at,
            };

            (OK_200, "application/json", format!("{{ \"value\": \"{}\" }}", body.to_string()))
        }
        // 404
        _ => (NOT_FOUND_404, "text/plain", "".to_string()),
    };

    let response = format!("HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}", status, content_type, message_body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
