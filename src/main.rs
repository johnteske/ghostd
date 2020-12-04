use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(120);

include!(concat!(env!("OUT_DIR"), "/html.rs"));

const OK_200: &str = "200 OK";
const NOT_FOUND_404: &str = "404 NOT FOUND";

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4321").unwrap();

    let state = Arc::new(Mutex::new("".to_string()));
    let state_setter_tx = start_state_setter(Arc::clone(&state));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, state.clone(), state_setter_tx.clone());
    }
}

fn handle_connection(
    mut stream: TcpStream,
    state: Arc<Mutex<String>>,
    state_setter_tx: Sender<String>,
) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status, content_type, message_body): (&str, &str, String) = match buffer {
        // assets
        b if b.starts_with(b"GET / ") => (OK_200, "text/html", HTML.to_string()),
        // values
        b if b.starts_with(b"GET /value ") => {
            //let value = *state.lock().unwrap();
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

            state_setter_tx.send(body.to_string()).unwrap();

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

// this wipes state on TIMEOUT--
// but also runs at an interval of TIMEOUT
// (or less if it receives a new message)
fn start_state_setter(state: Arc<Mutex<String>>) -> Sender<String> {
    let (tx, rx) = channel::<String>();
    thread::spawn(move || loop {
        let new_value = rx.recv_timeout(TIMEOUT).unwrap_or_default();
        let mut value = state.lock().unwrap();
        println!("state setter: {} => {}", value, new_value);
        *value = new_value;
    });
    tx
}
