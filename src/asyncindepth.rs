use std::io::Result;
use std::net::TcpListener;
use std::os::unix::prelude::AsRawFd;

/// https://youtu.be/_3LpJ6I-tzc?t=427

pub fn asyncmain() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;
    listener.set_nonblocking(true)?;

    let fd = listener.as_raw_fd();
    let mut polls_fds = vec![libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    }];
}

fn wait(fds: &mut [libc::pollfd]) -> Result<usize> {
    syscall!(poll(
        fds.as_mut_ptr() as *mut libc::pollfd,
        fds.len() as libc::nfds_t,
        -1
    ))
}

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
