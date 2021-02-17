//! A simple single-threaded design
//!
//! Run this example server and
//! test the connection with `curl localhost:4321`

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
                // TODO parse, get method, path, body (content-length header, content text type)
                let start_line = parse_start_line(stream);
                println!("{}", start_line);

                match start_line.get(0..4).unwrap() {
                    "GET " => {
                        // 200 OK
                        // 404 empty or 410 GONE
                        println!("state:\t{}", state.get());
                    }
                    "POST" => {
                        // 204 no content
                        // 411 length required
                        // 413 payload too large
                        // 415 unsupported media type
                        state.set("something".to_string());
                    }
                    // 404 path--or 400?
                    // http 501 not impl
                    _ => {}
                }

                //stream.write(response.as_bytes()).unwrap();
                //stream.flush().unwrap();
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
}
