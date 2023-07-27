use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) -> (i32, char, std::net::TcpListener) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [1, 2, 3, 4];
                    stream.read(&mut buffer);
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
