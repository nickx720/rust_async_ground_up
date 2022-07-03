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
