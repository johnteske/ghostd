use std::thread;
use std::time::Duration;

mod server;
mod state;

const TICK_RATE: Duration = Duration::from_millis(500);
const MAX_ELAPSED: Duration = Duration::from_secs(5);

fn main() {
    let mut state = state::State::new(MAX_ELAPSED);
    let server = server::Server::new("127.0.0.1:4321");

    loop {
        state.clear_if_expired();
        server.handle_nonblocking(&mut state);
        thread::sleep(TICK_RATE);
    }
}
