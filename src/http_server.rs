use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::request::Request;

pub struct HttpServer {}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {}
    }

    pub fn serve(&self) {
        let listener = TcpListener::bind("127.0.0.1:3000").expect("could not bind to port 3000");

        for tcp_stream in listener.incoming() {
            match tcp_stream {
                Ok(tcp_stream) => {
                    println!("Got a connection");
                    self.handle_request(tcp_stream);
                }
                Err(e) => println!("{:#?}", e),
            }
        }
    }

    fn handle_request(&self, stream: TcpStream) {
        let reader = BufReader::new(stream);
        let request = Request::from(reader);

        println!("Asada {:#?}", request)
    }
}
