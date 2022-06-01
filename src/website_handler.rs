use std::fs;
use super::server::Handler;
use super::http::{Request,Response,StatusCode,Method};

pub struct  WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}",&self.public_path,file_path);
        let public_path = fs::canonicalize( &self.public_path).unwrap() ;
        match fs::canonicalize(path) {
            Ok(path) => {
                println!("{}",path.to_string_lossy());
                if path.starts_with(public_path) {
                    fs::read_to_string(path).ok()
                }
                else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler  {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => {
                        Response::new(StatusCode::Ok,Some(content))
                    },
                    None => {
                        Response::new(StatusCode::NotFound, None)
                    },
                },
            },
            _ => Response::new(StatusCode::NotFound,None),
        }
    }
}