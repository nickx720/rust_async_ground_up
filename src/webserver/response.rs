use std::io::Result;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};
use std::net::TcpStream;

pub struct Response {
    writer: BufWriter<TcpStream>,
}

impl Response {
    pub fn new(client: TcpStream) -> Self {
        Response {
            writer: BufWriter::new(client),
        }
    }

    pub fn write_status(&mut self, code: i32, status: &str) -> Result<usize> {
        self.writer
            .write(format!("HTTP/1.1 {} {}\n", code, status).as_bytes())
    }

    pub fn write_header(&mut self, key: &str, val: &str) -> Result<usize> {
        self.writer
            .write(format!("\"{}\": {}\n", key, val).as_bytes())
    }

    pub fn write_body(&mut self, val: &[u8]) -> Result<usize> {
        self.write_header("content-length", val.len())?;
        self.writer.write(b"\n")?;
        self.writer.write(val)
    }
}
