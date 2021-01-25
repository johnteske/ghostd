use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

include!(concat!(env!("OUT_DIR"), "/html.rs"));

pub fn connection(mut stream: TcpStream, tx: Sender<String>, state: Arc<Mutex<String>>) {
    let mut buffer = [0; 1024];
    /*let size =*/
    stream.read(&mut buffer).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let result = req.parse(&buffer).unwrap();
    match result {
        httparse::Status::Complete(p) => {
            println!("{}", String::from_utf8(buffer[0..p].to_vec()).unwrap());
            // POST body
            //println!("{:?}", &buffer[p..]);
        }
        httparse::Status::Partial => {}
    }
    //if let result
    //   println!("{:?}", result);
    //   println!("{}", result.ok());
    //    let (status, content_type, message_body): (&str, &str, String) = match buffer {
    //        // assets
    //        b if b.starts_with(b"GET / ") => (OK_200, "text/html", HTML.to_string()),
    //        // values
    //        b if b.starts_with(b"GET /value ") => {
    //            let value = state.lock().unwrap();
    //
    //            (OK_200, "text/plain", value.to_string())
    //        }
    //        b if b.starts_with(b"POST /value ") => {
    //            let s = String::from_utf8(b[0..size].to_vec()).expect("error converting request");
    //            let body = s.lines().last().expect("error getting request body");
    //
    //            tx.send(body.to_string()).unwrap();
    //
    //            (OK_200, "text/plain", body.to_string())
    //        }
    //        // 404
    //        _ => (NOT_FOUND_404, "text/plain", "".to_string()),
    //    };
    //
    //    let response = format!(
    //        "HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}",
    //        status, content_type, message_body
    //    );
    //
    //    stream.write(response.as_bytes()).unwrap();
    //    stream.flush().unwrap();
}
