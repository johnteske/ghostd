use std::io::prelude::*;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(3);

fn timer_new(rx: Arc<Mutex<Receiver<bool>>>, state: Arc<Mutex<String>>) {
    println!("timer_new");
    thread::spawn(move || loop {
        match rx.lock().unwrap().recv_timeout(TIMEOUT) {
            Ok(false) | Err(_) => {
                println!("wipe it");
                let mut value = state.lock().unwrap();
                *value = "".to_string();
                break;
            }
            _ => {}
        }
    });
}

fn main() {
    let state = Arc::new(Mutex::new(String::new()));

    let (tx, rx) = channel::<bool>();
    let rx = Arc::new(Mutex::new(rx));

    for line in std::io::stdin().lock().lines() {
        let mut old_value = state.lock().unwrap();
        let new_value = line.unwrap();
        match (old_value.is_empty(), new_value.is_empty()) {
            (_, true) => {
                println!("new_value is empty. clear state, ignore timer");
                tx.send(false).unwrap(); // TODO do not ignore timer--kill it (if it exists)
            }
            (true, false) => {
                println!("old_value is empty. start timer");
                timer_new(Arc::clone(&rx), Arc::clone(&state));
            }
            (false, false) => {
                println!("old_value is not empty. send msg to restart timer");
                tx.send(true).unwrap();
            }
        }

        *old_value = new_value;
    }
}
