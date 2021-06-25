use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

pub type Db = Arc<Mutex<State>>;

pub struct State {
    value: String,
    created_at: Option<Instant>,
    ttl: Duration,
}

impl State {
    pub fn new(ttl: Duration) -> State {
        State {
            value: String::new(),
            created_at: None,
            ttl,
        }
    }
    pub fn get(&self) -> &String {
        &self.value
    }
    pub fn set(&mut self, new_value: String) {
        self.value = new_value;
        self.created_at = Some(Instant::now());
    }
    fn clear(&mut self) {
        self.value.clear();
        self.created_at = None;
    }
    pub fn clear_if_expired(&mut self) {
        if let Some(ts) = self.created_at {
            if ts.elapsed() >= self.ttl {
                self.clear();
            }
        }
    }
}
