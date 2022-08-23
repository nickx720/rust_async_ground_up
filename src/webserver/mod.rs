use std::io::Error;
use std::io::Result;
use std::net::TcpListener;
use std::sync::Arc;
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

fn waitpid(pid: i32) -> Result<u32> {
    check_err(unsafe { libc::waitpid(pid, 0 as *mut libc::c_int, 0) }).map(|code| code as u32)
}

pub fn webservermain() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;

    let pid = std::process::id();
    println!("[{pid}] server listening on 127.0.0.1:7000");

    let mut router = Router::new();
    routes::configure(&mut router);
    let router = Arc::new(router);
    let mut pids = Vec::new();
    for _ in 0..10 {
        let child_pid = fork()?;
        // complete this
        if child_pid == 0 {
            let mut handles = Vec::new();
            for client in listener.incoming() {
                let router = Arc::clone(&router);
                let handle = std::thread::spawn(move || {
                    println!(
                        "[{pid}] {:?} client connected at",
                        std::thread::current().id()
                    );
                    router.route_client(client.unwrap());
                });
                handles.push(handle);
            }
            while let Some(handle) = handles.pop() {
                handle.join().unwrap();
            }
            break;
        } else {
            println!("[{pid}] forking process,new {child_pid}");
        }
        pids.push(child_pid);
    }
    // Parent process exits here
    for p in pids {
        waitpid(p as i32)?;
        println!("[{pid}] <<< {p} exited")
    }
    Ok(())
}
