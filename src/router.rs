use std::collections::HashMap;

/// https://youtu.be/fdxhcDne2Ww?t=192

#[derive(PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Router {}
    }

    pub fn insert(&mut self, method: &str, path: &str) {}

    pub fn handle(&self, method: &str, resource: &str, client: TcpStream) {}
}
