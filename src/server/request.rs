use http::header;
use std::str::FromStr;

use super::BUFFER_SIZE;

pub enum Request {
    GET { path: String },
    POST { path: String, body: String },
}

#[derive(Debug)]
pub enum ParseError {
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

// TODO
pub fn parse(buffer: [u8; BUFFER_SIZE]) -> Result<Request, ParseError> {
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
