fn main() {
    let get = Method::GET("abcd".to_string());
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }
    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: Option<u8>,
    method: Method,
}

enum Method {
    GET(String),
    POST(u64),
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */
