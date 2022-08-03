use crate::webserver::router::Router;
use std::io::Result;
use std::net::TcpStream;

use super::response::Response;

pub fn configure(router: &mut Router) {
    router.insert(super::router::Method::GET, "/", index);
}

fn index(client: TcpStream) -> Result<()> {
    let res = Response::new(client);
    res.sendfile(200, "static/index.html")
}
