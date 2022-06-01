use std::{net::TcpListener, io::{Read}};
use crate::http::{Request,Response, StatusCode, ParseError};
use std::convert::{TryFrom};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        Response::new(
            StatusCode::Ok,
             Some("<h1>Hello World</h1>".to_string()))
    }
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Parsed Error is {}",e);
        Response::new(
            StatusCode::BadRequest,
             None
        )
    }
}

pub struct  Server {
    addr: String,
}

impl  Server {
    pub fn new(addr: String) -> Self {
        Self { 
            addr
        }
    }
    pub fn run(self,mut handler: impl Handler) {
        println!("Listening on {}",self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Connection is established {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                              Ok(req) => {
                                  handler.handle_request(&req)
                                  
                              },
                              Err(err) => {
                                handler.handle_bad_request(&err)
                              },     
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send request {}",e);
                            }
                        },
                        Err(_e) => println!("Failed to read from connection: {}", _e),
                    };
                }
                Err(_err) => {
                    println!("Err");
                }
            }

        }
    }
}


