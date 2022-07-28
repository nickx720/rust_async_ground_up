use std::io::Result;
use std::net::TcpListener;
mod node;
pub mod router;

pub fn webservermain() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1.7000")?;

    for client in listener.incoming() {}
    todo!()
}
