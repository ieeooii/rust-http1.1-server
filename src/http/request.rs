use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    query_string: Option<u8>,
    method: Method,
}
/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

impl TryFrom<&[u8]> for Request {
    type Error = String;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
