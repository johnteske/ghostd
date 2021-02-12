//! A simple single-threaded design
//!
//! Run this example server and
//! test the connection with `curl localhost:4321`
//!
//! Learnings/log:
//!
//! 1. Nonblocking stdin, for purposes of this example, would require another thread
//! While this isn't particularly an issue in this example, it would add extraneous
//! code--and I may as well use a TcpListener so the example code looks more like
//! the end goal for clarity.
//!
//! 2. Timeout. At first I had thought about setting an Instant and adding an duration
//! for the expiration Instant, then would compare against Instant::now(). Reading the
//! documentation it looks like simple comparisons are supported, so only the max
//! lifetime duration and timestamp are needed.
//!
//! 3. Error handling. I wanted to add a guard to the error checking to ensure I was
//! only permitting expected errors. For the purposes of this example, any other error
//! panics.
//!
//! 4. Parsing the request. The next step for me is to get and set state, for which I
//! need to know the HTTP method. I added parse_start_line, not as a meaningful fn but
//! mainly to abstract the parsing out of the main loop. In the real implementation it
//! will be replaced by robust parsing.
//!
//! 5. State. With this single-threaded model, I don't believe a lock is required.

use std::io::{ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const TICK_RATE: Duration = Duration::from_secs(1);
const MAX_ELAPSED: Duration = Duration::from_secs(5);

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4321").unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut state = state::State::new(MAX_ELAPSED);

    loop {
        match listener.accept() {
            Ok((stream, _addr)) => {
                let start_line = parse_start_line(stream);
                println!("{}", start_line);

                match start_line.get(0..4).unwrap() {
                    "GET " => {
                        println!("state:\t{}", state.get());
                    }
                    "POST" => {
                        state.set("something".to_string());
                    }
                    _ => {}
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => panic!("{}", e),
        }

        state.check();

        thread::sleep(TICK_RATE);
    }
}

fn parse_start_line(mut stream: TcpStream) -> String {
    let mut buffer = [0; 32];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8(buffer.to_vec()).unwrap();
    let start_line = request.lines().next().unwrap();
    start_line.to_string()
}

mod state {
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
            self.value.clear();
        }
        pub fn check(&mut self) {
            // check or clear
            if let Some(ts) = self.timestamp {
                println!("elapsed: {}", ts.elapsed().as_secs());
                if ts.elapsed() >= self.max_elapsed {
                    println!("reached MAX_ELAPSED");
                    self.clear();
                    self.timestamp = None;
                }
            }
        }
    }
}
