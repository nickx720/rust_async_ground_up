use crate::webserver::router::Router;
use std::io::Result;
use std::net::TcpStream;

pub fn configure(router: &mut Router) {
    router.insert("GET", "/", index);
}

fn index(client: TcpStream) -> Result<()> {}
