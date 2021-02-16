use std::io::{ErrorKind, Read};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use super::state::State;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(addr: impl ToSocketAddrs) -> Server {
        let listener = TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();

        Server { listener }
    }
    pub fn handle_nonblocking(&self, state: &mut State) {
        match self.listener.accept() {
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
                        state.set("TODO body".to_string());
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
    }
}

fn parse_start_line(mut stream: TcpStream) -> String {
    let mut buffer = [0; 32];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8(buffer.to_vec()).unwrap();
    let start_line = request.lines().next().unwrap();
    start_line.to_string()
}
