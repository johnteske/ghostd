use std::io::prelude::*;
use std::io::{ErrorKind, Read};
use std::net::{TcpListener, ToSocketAddrs};
use http::Response;

use super::state::State;

mod request;
use request::Request;

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

                let result = request::parse(buffer);
                let mut response = Response::builder();

                match result {
                    Ok(Request::GET { path: _ }) => {
                        response = response.status(http::StatusCode::OK);
                        // 200 OK
                        // 404 empty or 410 GONE
                        println!("state:\t{}", state.get());
                    }
                    Ok(Request::POST { body, .. }) => {
                        // 204 no content
                        // 411 length required
                        // 413 payload too large
                        // 415 unsupported media type
                        println!("state:\t{}", body);
                        state.set(body);
                    } // 404 path--or 400?
                    // http 501 not impl
                    Err(_) => {
                        println!("parse error!"); // TODO be specific with reply
                    }
                }

                // send response
                println!("{:?}", response);
                println!("{:?}", response.body(()).unwrap());
                //println!("{}", response.body("asd").unwrap());
                //println!("{}", response.body(()).unwrap().as_bytes());
//                stream.write(response.as_bytes()).unwrap();
//                stream.flush().unwrap();
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => panic!("{}", e),
        }
    }
}
