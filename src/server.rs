// use http::{Response, StatusCode};
use std::io::{ErrorKind, Read};
use std::net::{TcpListener, ToSocketAddrs};

use super::state::State;

const BUFFER_SIZE: usize = 1024;

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
            Ok((mut stream, _addr)) => {
                let mut buffer = [0; BUFFER_SIZE];
                stream.read(&mut buffer).unwrap();

                let result = parse(buffer).unwrap(); // TODO handle parse error
                println!("{:?}", result);

                match result.method.as_str() {
                    "GET" => {
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

                // let response = Response::builder().body("TODO".to_string());
                //stream.write(response.as_bytes()).unwrap();
                //stream.flush().unwrap();
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => panic!("{}", e),
        }
    }
}

// TODO &str with lifetime, not String
#[derive(Debug)]
struct Parsed {
    method: String,
    path: String,
    content_type: Option<String>,
    content_length: Option<String>,
}

fn parse(buffer: [u8; BUFFER_SIZE]) -> Result<Parsed, String> {
    let mut headers = [httparse::EMPTY_HEADER; 8];
    let mut req = httparse::Request::new(&mut headers);
    let result = req.parse(&buffer).unwrap();

    if result.is_partial() {
        return Err("not complete".to_string());
    }

    Ok(Parsed {
        method: req.method.unwrap().to_string(), // TODO this is required
        path: req.path.unwrap().to_string(),     // TODO also required
        content_type: parse_header(req.headers.iter().find(|&&h| h.name == "Content-Type")),
        content_length: parse_header(req.headers.iter().find(|&&h| h.name == "Content-Length")),
    })
}

fn parse_header(header: Option<&httparse::Header>) -> Option<String> {
    match header {
        Some(h) => String::from_utf8(h.value.to_vec()).ok(),
        None => None,
    }
}
