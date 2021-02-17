// use http::{Response, StatusCode};
use std::io::{ErrorKind, Read};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

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
            Ok((stream, _addr)) => {
                // TODO parse, get method, path, body (content-length header, content text type)
                let result = parse(stream);
                println!("{:?}", result);

                let start_line = "TODO";
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

                // let response = Response::builder().body("TODO".to_string());
                //stream.write(response.as_bytes()).unwrap();
                //stream.flush().unwrap();
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => panic!("{}", e),
        }
    }
}

// ideally not strings, no copy
#[derive(Debug)]
struct Parsed {
    method: String,
    path: String,
    content_type: String,
    content_length: String,
}

fn parse(mut stream: TcpStream) -> Result<Parsed, String> {
    let mut buffer = [0; BUFFER_SIZE];
    stream.read(&mut buffer).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 8];
    let mut req = httparse::Request::new(&mut headers);
    let result = req.parse(&buffer).unwrap();

    if result.is_partial() {
        return Err("not copmlete".to_string());
    }

    Ok(Parsed {
        method: req.method.unwrap().to_string(),
        path: req.path.unwrap().to_string(),
        content_type: String::from_utf8(
            req.headers
                .iter()
                .find(|&&h| h.name == "Content-Type")
                .unwrap() // TODO
                .value
                .to_vec(),
        )
        .unwrap(),
        content_length: String::from_utf8(
            req.headers
                .iter()
                .find(|&&h| h.name == "Content-Length")
                .unwrap() // TODO keep this as Option 411 error for POST
                .value
                .to_vec(),
        )
        .unwrap(),
    })
}
