use std::io::Write;
use std::net::TcpStream;

use network_essentials::web_api::api_register::ApiRegister;
use network_essentials::web_api::api_register::HttpConnectionDetails;
use network_essentials::web_api::http_server::HttpServer;

fn main() {
    println!("Hello World!");

    let mut default_api = ApiRegister::new(error_404);
    default_api.register_function("GET", "/", index);

    let mut files_api = ApiRegister::new(error_404);
    files_api.register_function("GET", "/", index_files);

    default_api.register_prefix("/files", files_api);

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

fn index_files(http_request: HttpConnectionDetails, mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200";
    let contents = format!("{:?}", http_request);
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn error_404(http_request: HttpConnectionDetails, mut stream: TcpStream) {
    let status_line = "HTTP/1.1 400";
    let contents = format!("{:?}", http_request);
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}