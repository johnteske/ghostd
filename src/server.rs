// use http::{Response, StatusCode};
use std::io::{ErrorKind, Read};
use std::net::{TcpListener, ToSocketAddrs};
use std::str::FromStr;

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
                                                     //println!("{:?}", result);

                match result {
                    Request::GET { path } => {
                        // 200 OK
                        // 404 empty or 410 GONE
                        println!("state:\t{}", state.get());
                    }
                    Request::POST { .. } => {
                        // 204 no content
                        // 411 length required
                        // 413 payload too large
                        // 415 unsupported media type
                        //println!("state:\t{}", result.body);
                        state.set("TODO body".to_string());
                    } // 404 path--or 400?
                      // http 501 not impl
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

enum Request {
    GET { path: String },
    POST { path: String, body: String },
}

fn parse(buffer: [u8; BUFFER_SIZE]) -> Result<Request, &'static str> {
    let mut headers = [httparse::EMPTY_HEADER; 8];
    let mut req = httparse::Request::new(&mut headers);
    let result = req.parse(&buffer).unwrap();

    let path = req.path.ok_or("error parsing path")?.to_string();

    match req.method.unwrap() {
        "GET" => Ok(Request::GET { path }),
        "POST" => {
            let content_length =
                parse_header(req.headers.iter().find(|&&h| h.name == "Content-Length")).unwrap(); // TODO error if not
            let len = usize::from_str(&content_length).unwrap();
            let body_byte_offset = result.unwrap(); // panics if is_partial
            let body = String::from_utf8(buffer[body_byte_offset..len].to_vec()).unwrap();
            Ok(Request::POST { path, body })
        }
        _ => Err("TODO"),
    }
}

fn parse_header(header: Option<&httparse::Header>) -> Option<String> {
    match header {
        Some(h) => String::from_utf8(h.value.to_vec()).ok(),
        None => None,
    }
}
