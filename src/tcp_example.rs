use anyhow::Result;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub fn tcp_main() -> Result<()> {
    let listener = TcpListener::bind("localhost:3000")?;
    for connection in listener.incoming() {
        let stream: TcpStream = connection?;
        /// Moving handle_connection to inside thread::spawn makes it multi-threaded
        std::thread::spawn(|| handle_connection(stream));
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut request = Vec::new();
    let mut reader = BufReader::new(&mut stream);
    reader.read_until(b'\n', &mut request)?;

    let request = String::from_utf8(request)?; // convert bytes to human readable format
    print!("HTTP request line: {}", request);

    let response = concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Length : 121 \r\n",
        "Content-Type: text/html \r\n\r\n",
        "<!DOCTYPE html> <html> <body> <h1>Enter a search term below:</h1> </body> </html>"
    );

    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
