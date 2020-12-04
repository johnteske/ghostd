use std::net::TcpListener;
use std::sync::{Arc, Mutex};

mod handler;
mod tmp_state;

fn main() {
    let state = Arc::new(Mutex::new("".to_string()));
    let tx = tmp_state::start(Arc::clone(&state));

    let listener = TcpListener::bind("0.0.0.0:4321").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handler::connection(stream, tx.clone(), state.clone());
    }
}
