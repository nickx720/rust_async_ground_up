use crate::webserver::node::Node;
use crate::webserver::response::Response;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Result};
use std::net::TcpStream;

pub type HandlerFn = fn(TcpStream) -> Result<()>;

/// https://youtu.be/fdxhcDne2Ww?t=1062

#[derive(PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct Router {
    routes: HashMap<Method, Node>,
}

impl Router {
    pub fn new() -> Self {
        Router {routes: HashMap::new()}
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
        if len == 0 {
            return Ok(());
        }
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() < 2 {
            let res = Response::new(client);
            res.sendfile(400, "static/_400.html")
        } else {
            match (parts[0], parts[1]) {
                ("GET", path) => self.handle(Method::GET, path, client),
                _ => {
                    let res = Response::new(client);
                    res.sendfile(404, "static/_404.html")
                },
            }
        }
    }

    pub fn insert(&mut self, method: Method, path: &str, handler: HandlerFn) {
        let node = self.routes.entry(method).or_insert(Node::new("/"));
        node.insert(path, handler);
    }

    pub fn handle(&self, method: Method, path: &str, client: TcpStream) -> Result<()> {
        if let Some(node)
    }
}
