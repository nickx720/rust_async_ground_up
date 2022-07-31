use crate::webserver::node::Node;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Result};
use std::net::TcpStream;

pub type HandlerFn = fn(TcpStream) -> Result<()>;

/// https://youtu.be/fdxhcDne2Ww?t=685

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

    pub fn route_client(&self, client: TcpStream) -> Result<()> {
        let mut reader = BufReader::new(&client);
        let buf = reader.fill_buf()?;

        // read a single line (if one exists)
        let mut line = String::new();
        let mut line_reader = BufReader::new(buf);
        let len = line_reader.read_line(&mut line)?;

        // consume bytes read from original reader
        reader.consume(len);
    }

    pub fn insert(&mut self, method: &str, path: &str, handler: HandlerFn) {
        let node = self.routes.entry(method).or_insert(Node::new("/"));
        node.insert(path, handler);
    }

    pub fn handle(&self, method: &str, resource: &str, client: TcpStream) -> Result<()> {}
}
