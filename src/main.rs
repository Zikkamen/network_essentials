use std::io::Write;
use std::net::TcpStream;
use std::collections::HashMap;

use network_essentials::web_api::api_register::ApiRegister;
use network_essentials::web_api::api_register::HttpConnectionDetails;
use network_essentials::web_api::http_server::HttpServer;

fn main() {
    println!("Hello World!");

    let mut default_api = ApiRegister::new(index);
    default_api.register_function("POST", "/", index);

    let http_server = HttpServer::new("127.0.0.1:7878", default_api);
    http_server.start_listening();
}

fn index(http_request: HttpConnectionDetails, mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200";
    let contents = format!("{:?}", http_request);
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}