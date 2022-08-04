use std::fs::File;
use std::io::Result;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};
use std::net::TcpStream;

pub struct Response {
    writer: BufWriter<TcpStream>,
}

pub fn status(code: i32) -> &'static str {
    match code {
        200 => "OK",
        400 => "BAD REQUEST",
        404 => "NOT FOUND",
        _ => "NOT IMPLEMENTED",
    }
}

impl Response {
    pub fn new(client: TcpStream) -> Self {
        Response {
            writer: BufWriter::new(client),
        }
    }

    pub fn write_status(&mut self, code: i32) -> Result<usize> {
        self.writer
            .write(format!("HTTP/1.1 {} {}\n", code, status(code)).as_bytes())
    }

    pub fn write_header(&mut self, key: &str, val: &str) -> Result<usize> {
        self.writer
            .write(format!("\"{}\": {}\n", key, val).as_bytes())
    }

    pub fn write_body(&mut self, val: &[u8]) -> Result<usize> {
        self.write_header("content-length", &val.len().to_string())?;
        self.writer.write(b"\n")?;
        self.writer.write(val)
    }

    pub fn parse_mime_type(&self, key: &str) -> &str {
        if let Some((_, ext)) = key.rsplit_once(".") {
            match ext {
                "html" => "text/html",
                "css" => "text/css",
                "js" => "text/javascript",
                "jpg" => "image/jpeg",
                "jpeg" => "image/jpeg",
                "png" => "image/png",
                "ico" => "image/x-icon",
                "pdf" => "application/pdf",
                _ => "text/plain",
            }
        } else {
            "text/plain"
        }
    }

    pub fn write_file(&mut self, path: &str) -> Result<usize> {
        let file = File::open(path)?;
        let mut buf = Vec::new();
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut buf)?;

        self.write_header(
            "content-type",
            format!("{};charset=UTF-8", self.parse_mime_type(path)).as_str(),
        )?;
        self.write_body(&buf)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

    pub fn sendfile(&mut self, code: i32, path: &str) -> Result<()> {
        self.write_status(code)?;
        self.write_file(path)?;
        self.flush()
    }
}
