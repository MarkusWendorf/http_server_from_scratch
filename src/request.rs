use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Lines, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    fn parse_request_line(&mut self, line: String) {
        for line_segment in line.split_whitespace().enumerate() {
            match line_segment {
                (0, method) => self.method = method.to_owned(),
                (1, uri) => self.uri = uri.to_owned(),
                (2, http_version) => self.http_version = http_version.to_owned(),
                (_, _) => {}
            }
        }
    }

    fn parse_header(&mut self, line: String) {
        if let Some((header, value)) = line.split_once(":") {
            self.headers
                .insert(header.trim().to_owned(), value.trim().to_owned());
        }
    }
}

impl From<BufReader<TcpStream>> for Request {
    fn from(mut reader: BufReader<TcpStream>) -> Self {
        let mut request = Self::default();
        let lines = reader.by_ref().lines();

        for line in lines.enumerate() {
            match line {
                (0, Ok(line)) => request.parse_request_line(line),
                (_, Ok(line)) if line == "" => break,
                (_, Ok(line)) => request.parse_header(line),
                (_, _) => {}
            }
        }

        let content_length = match request.headers.get("Content-Length") {
            Some(length) => length.parse::<usize>().unwrap_or_default(),
            None => 0,
        };

        let mut buffer = vec![0u8; content_length];
        match reader.read_exact(&mut buffer) {
            Ok(_) => request.body = Some(buffer),
            Err(_) => { /* Todo error handling */ }
        }

        request
    }
}

impl Default for Request {
    fn default() -> Self {
        Request {
            method: "GET".to_owned(),
            uri: "/".to_owned(),
            http_version: "HTTP/1.1".to_owned(),
            headers: HashMap::new(),
            body: None,
        }
    }
}
