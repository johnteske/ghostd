use http::{header, Response, StatusCode};
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
                    Request::GET { path: _ } => {
                        // 200 OK
                        // 404 empty or 410 GONE
                        println!("state:\t{}", state.get());
                    }
                    Request::POST { body, .. } => {
                        // 204 no content
                        // 411 length required
                        // 413 payload too large
                        // 415 unsupported media type
                        //println!("state:\t{}", result.body);
                        state.set(body);
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

#[derive(Debug)]
enum ParseError {
    PartialRequest,
    Path,
    ContentLength,
    ContentType,
    Method,
}

impl From<httparse::Error> for ParseError {
    fn from(_: httparse::Error) -> ParseError {
        ParseError::PartialRequest
    }
}

fn parse(buffer: [u8; BUFFER_SIZE]) -> Result<Request, ParseError> {
    let mut headers = [httparse::EMPTY_HEADER; 8];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buffer)?;

    let path = req.path.ok_or(ParseError::Path)?.to_string();
    let mut headers = http::HeaderMap::new();

    match req.method.unwrap() {
        "GET" => Ok(Request::GET { path }),
        "POST" => {
            for header in req.headers {
                match header::HeaderName::from_str(header.name) {
                    Ok(header_name) if header_name == header::CONTENT_LENGTH => {
                        headers.insert(
                            header_name,
                            header::HeaderValue::from_bytes(header.value).unwrap(),
                        );
                    }
                    _ => continue,
                }
            }

            // let content_length = headers.get(header::CONTENT_LENGTH).ok_or(ParseError::ContentLength)?.to_str().ok_or(ParseError::ContentLength)?;
            // let body_byte_offset = result.unwrap(); // panics if is_partial
            // let body = String::from_utf8(buffer[body_byte_offset..len].to_vec()).unwrap();
            let body = "TODO".to_string();
            Ok(Request::POST { path, body })
        }
        _ => Err(ParseError::Method),
    }
}
