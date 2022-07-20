use std::collections::HashMap;
use std::io::Result;
use std::net::TcpListener;
use std::os::unix::prelude::AsRawFd;

/// https://youtu.be/_3LpJ6I-tzc?t=569

pub fn asyncmain() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;
    listener.set_nonblocking(true)?;

    let fd = listener.as_raw_fd();
    let mut polls_fds = vec![libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    }];
    let mut handlers: HashMap<i32, Box<dyn Fn()>> = HashMap::new();
    handlers.insert(
        fd,
        Box::new(move || {
            let (client, addr) = listener.accept().unwrap();
            handle_connection(client, addr);
        }),
    );
    loop {
        let num_events = wait(&mut polls_fds)?;
        if num_events > 0 {
            for poll_fd in &polls_fds {
                if poll_fd.revents & libc::POLLIN != 0 {
                    if let Some(handler) = handlers.get(&poll_fd.fd) {
                        handler()
                    }
                    println!(
                        "poll_fd {} received revents {}",
                        poll_fd.fd, poll_fd.revents
                    );
                    let (client, addr) = listener.accept().unwrap();
                    handle_connection(client, addr);
                }
            }
        }
    }
}

fn wait(fds: &mut [libc::pollfd]) -> Result<usize> {
    loop {
        match syscall!(poll(
            fds.as_mut_ptr() as *mut libc::pollfd,
            fds.len() as libc::nfds_t,
            -1
        )) {
            Ok(n) => break Ok(n as usize),
            Err(e) if e.raw_os_err() == Some(libc::EAGAIN) => continue,
            Err(e) => return Err(e),
        }
    }
}

#[macro_use]
macro_rules! syscall {
    ($fn: ident $args:tt) => {
        let res = unsafe {libc::$fn $args};
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }

    };
}
