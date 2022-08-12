use std::io::Error;
use std::io::Result;
use std::net::TcpListener;
mod node;
mod response;
mod router;
mod routes;

use router::Router;

fn check_err(num: i32) -> Result<i32> {
    if num < 0 {
        return Err(Error::last_os_error());
    }
    Ok(num)
}

fn fork() -> Result<u32> {
    check_err(unsafe { libc::fork() }).map(|pid| pid as u32)
}

pub fn webservermain() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;

    let pid = std::process::id();
    println!("[{pid}] server listening on 127.0.0.1:7000");

    let mut router = Router::new();
    routes::configure(&mut router);

    for client in listener.incoming() {
        let chiild_pid = fork()?;
        if chiild_pid == 0 {
            router.route_client(client?);
        } else {
            println!("[{pid} forking process, new {chiild_pid}]");
        }
    }
    Ok(())
}
