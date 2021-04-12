use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // 1024 might not be enough
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // creates a slice that contains the entire array [..]
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // Request must implement the debug trait to be logged to the console
                                    // rustc provide basic impl for some traits, like debug - use the derive attr
                                    dbg!(request);
                                    let response = Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>it works!</h1>".to_string()),
                                    );
                                    // in order to write the response to the stream, it must implement display
                                    write!(stream, "{}", response);
                                }
                                Err(err) => println!("There was an error: {}", err),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
