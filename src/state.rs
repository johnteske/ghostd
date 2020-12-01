use std::time::Instant;

pub struct State {
    pub value: String,
    pub expires_at: Instant,
}
