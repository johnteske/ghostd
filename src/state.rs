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
    pub fn get(&self) -> String {
        String::new()
    }
    pub fn set(&mut self, new_value: String) {
        self.timestamp = Some(Instant::now());
        self.value = new_value;
    }
    fn clear(&mut self) {
        self.timestamp = None;
        self.value.clear();
    }
    // check_or_clear
    pub fn check(&mut self) {
        if let Some(ts) = self.timestamp {
            println!("elapsed: {}", ts.elapsed().as_secs());
            if ts.elapsed() >= self.max_elapsed {
                println!("reached MAX_ELAPSED");
                self.clear();
            }
        }
    }
}
