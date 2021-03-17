use std::time::{Duration, Instant};

pub struct State {
    value: String,
    timestamp: Option<Instant>,
    max_elapsed: Duration,
}

impl State {
    pub fn new(max_elapsed: Duration) -> State {
        State {
            value: String::new(),
            timestamp: None,
            max_elapsed,
        }
    }
    pub fn get(&self) -> &String {
        &self.value
    }
    pub fn set(&mut self, new_value: String) {
        self.timestamp = Some(Instant::now());
        self.value = new_value;
    }
    fn clear(&mut self) {
        self.timestamp = None;
        self.value.clear();
    }
    pub fn clear_if_expired(&mut self) {
        if let Some(ts) = self.timestamp {
            if ts.elapsed() >= self.max_elapsed {
                self.clear();
            }
        }
    }
}
