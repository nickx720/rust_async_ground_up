use std::collections::HashMap;
use std::io::Result;
use std::net::TcpStream;

pub type HandlerFn = fn(TcpStream) -> Result<()>;

/// https://youtu.be/fdxhcDne2Ww?t=192

#[derive(PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct Router {
    routes: HashMap<Method, todo!()>,
}

impl Router {
    pub fn new() -> Self {
        Router {}
    }

    pub fn insert(&mut self, method: &str, path: &str, handler: HandlerFn) {}

    pub fn handle(&self, method: &str, resource: &str, client: TcpStream) -> Result<()> {}
}
