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
//! 3. Error handling. I wanted to add a guard to the error checking to ensure we were
//! only permitting expected errors. For the purposes of this example, any other error
//! panics.

use std::io::ErrorKind;
use std::net::TcpListener;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    const TICK_RATE: Duration = Duration::from_secs(1);
    const MAX_ELAPSED: Duration = Duration::from_secs(5);

    let listener = TcpListener::bind("127.0.0.1:4321").unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut timestamp: Option<Instant> = None;

    loop {
        match listener.accept() {
            Ok((_socket, addr)) => {
                timestamp = Some(Instant::now());
                println!("client: {:?}", addr)
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => panic!("{}", e),
        }

        if let Some(ts) = timestamp {
            println!("elapsed: {}", ts.elapsed().as_secs());
            if ts.elapsed() >= MAX_ELAPSED {
                println!("reached MAX_ELAPSED");
                timestamp = None;
            }
        }

        thread::sleep(TICK_RATE);
    }
}
