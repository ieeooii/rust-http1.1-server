fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }
        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {
    mod requrest {
        struct Request {
            path: String,
            query_string: Option<u8>,
            method: method::Method,
        }
    }

    mod method {
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
    }
}
/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */
