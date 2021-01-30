mod state {
    use std::sync::{Arc, Mutex};

    pub type State = Arc<Mutex<String>>;

    pub fn new() -> State {
        Arc::new(Mutex::new(String::new()))
    }
}

mod timer {
    use super::state::State;
    use std::sync::mpsc::Receiver;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    const TIMEOUT: Duration = Duration::from_secs(3);

    // TODO this change requires timer to own rx
    // pub fn channel() {
    // let (tx, rx) = channel::<bool>();
    // let rx = Arc::new(Mutex::new(rx));
    // (tx, rx)
    // }

    pub fn new(rx: Arc<Mutex<Receiver<bool>>>, state: State) {
        println!("timer start");
        thread::spawn(move || loop {
            match rx.lock().unwrap().recv_timeout(TIMEOUT) {
                Ok(false) | Err(_) => {
                    println!("timer end");
                    let mut value = state.lock().unwrap();
                    *value = "".to_string();
                    break;
                }
                _ => {}
            }
            println!("timer reset");
        });
    }
}

fn main() {
    use std::io::prelude::*;
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};

    let state = state::new();

    let (tx, rx) = channel::<bool>();
    let rx = Arc::new(Mutex::new(rx));

    for line in std::io::stdin().lock().lines() {
        let mut old_value = state.lock().unwrap();
        let new_value = line.unwrap();

        // Start, stop, or reset the timer
        match (old_value.is_empty(), new_value.is_empty()) {
            (_, true) => {
                // TODO these messages stack up,
                // which immediately wipes future writes
                tx.send(false).unwrap();
            }
            (true, false) => {
                timer::new(Arc::clone(&rx), Arc::clone(&state));
            }
            (false, false) => {
                tx.send(true).unwrap();
            }
        }

        // Update state
        *old_value = new_value;
    }
}
