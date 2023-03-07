use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)        
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    // Every struct has a Self which is an alasi to the struct.
    // So, Self is an alias to Server
    pub fn new(addr: String) -> Self {
        Self {
            // addr: addr
            // If parameter matches name of struct variable we can just use param
            // name directly and not do an assignment, as the compiler
            // will see that they are the same and just figure it out.
            // So, addr: addr is equivalent to
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        // loop is equivalent to while true

        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        
                            // to use the try_info we got for free by implement try_from:
                            // let res: &Result<Request, _> = &buffer[..].try_into();

                            // Request::try_from(&buffer as &[u8]); but better:
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from stream {}", e)
                    }
                },
                Err(e) => println!("Failed to establish a connection {}", e)
            }
        }
    }
}

