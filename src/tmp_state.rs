use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(60);

// https://github.com/johnteske/ghosty/issues/15
// this wipes state on TIMEOUT--
// but also runs at an interval of TIMEOUT
// (or less if it receives a new message)
pub fn start(state: Arc<Mutex<String>>) -> Sender<String> {
    let (tx, rx) = channel::<String>();
    thread::spawn(move || loop {
        let new_value = rx.recv_timeout(TIMEOUT).unwrap_or_default();
        let mut value = state.lock().unwrap();
        *value = new_value;
    });

    tx
}
