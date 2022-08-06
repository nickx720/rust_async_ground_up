use std::io::Result;
use std::net::TcpListener;
mod node;
mod response;
mod router;
mod routes;

use router::Router;

pub fn webservermain() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;

    let mut router = Router::new();
    routes::configure(&mut router);

    for client in listener.incoming() {
        router.route_client(client?);
    }
    Ok(())
}
