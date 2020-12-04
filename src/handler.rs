use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

include!(concat!(env!("OUT_DIR"), "/html.rs"));

const OK_200: &str = "200 OK";
const NOT_FOUND_404: &str = "404 NOT FOUND";

pub fn connection(
    mut stream: TcpStream,
    tx: Sender<String>,
    state: Arc<Mutex<String>>,
) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status, content_type, message_body): (&str, &str, String) = match buffer {
        // assets
        b if b.starts_with(b"GET / ") => (OK_200, "text/html", HTML.to_string()),
        // values
        b if b.starts_with(b"GET /value ") => {
            let value = state.lock().unwrap();

            (
                OK_200,
                "application/json",
                format!("{{ \"value\": \"{}\" }}", value),
            )
        }
        b if b.starts_with(b"POST /value ") => {
            let s = String::from_utf8(b.to_vec()).expect("error converting request");
            let s = s.trim_end_matches("\0"); // remove NUL
            let body = s.lines().last().expect("error getting request body");

            tx.send(body.to_string()).unwrap();

            (
                OK_200,
                "application/json",
                format!("{{ \"value\": \"{}\" }}", body.to_string()),
            )
        }
        // 404
        _ => (NOT_FOUND_404, "text/plain", "".to_string()),
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}",
        status, content_type, message_body
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
