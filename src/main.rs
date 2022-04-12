mod http_server;
mod request;
mod response;

use http_server::HttpServer;

fn main() {
    let server = HttpServer::new();
    server.serve();
}
