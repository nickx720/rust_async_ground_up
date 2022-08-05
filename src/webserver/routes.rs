use crate::webserver::router::Router;
use std::io::Result;
use std::net::TcpStream;

use super::response::Response;

pub fn configure(router: &mut Router) {
    router.insert(super::router::Method::GET, "/", index);
    router.insert(super::router::Method::GET, "/static/styles.css", styles);
}

fn index(client: TcpStream) -> Result<()> {
    let mut res = Response::new(client);
    res.sendfile(200, "static/index.html")
}

fn styles(client: TcpStream) -> Result<()> {
    let mut res = Response::new(client);
    res.sendfile(200, "static/styles.css")
}
