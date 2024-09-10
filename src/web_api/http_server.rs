use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    error,
    time::Duration,
};

use crate::web_api::api_register::ApiRegister;
use crate::web_api::api_register::HttpConnectionDetails;

pub struct HttpServer {
    address: String,
    connection_timeout: u64,
    worker_size: usize,
    api_register: ApiRegister,
}

impl HttpServer {
    pub fn new(address: &str,
            main_api_register: ApiRegister,
            worker_size: usize,
            connection_timeout: u64) -> Self {
        HttpServer{ 
            address: address.to_string(),
            worker_size: worker_size,
            api_register: main_api_register,
            connection_timeout: connection_timeout,
        }
    }

    pub fn start_listening(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        let max_read_time = Duration::from_millis(self.connection_timeout);

        let mut threads = vec![];

        for _i in 0..self.worker_size {
            let listener_clone = listener.try_clone().unwrap();
            let api_register_clone = self.api_register.clone();

            let j_handle = thread::spawn(move || {
                for stream in listener_clone.incoming() {
                    match stream {
                        Ok(mut stream) => {
                            let _ = stream.set_read_timeout(Some(max_read_time));

                            match handle_connection(&mut stream) {
                                Ok(hm) => {
                                    let response  = api_register_clone.handle_http_request(hm);

                                    stream.write_all(response.as_bytes()).unwrap();
                                    stream.flush().unwrap();
                                },
                                Err(e) => println!("Error handling incoming request {}", e),
                            }
                        },
                        Err(e) => println!("Error {e}"),
                    }
                }
            });

            threads.push(j_handle);
        }

        for i in threads {
            let _= i.join();
        }
    }
}

fn handle_connection(mut stream: &mut TcpStream) -> Result<HttpConnectionDetails,  Box<dyn error::Error + 'static>> {
    let mut buf_reader = BufReader::new(&mut stream);
    const BUF_SIZE:usize = 512;

    let mut buf_string = String::new();
    let mut i:usize = 0;

    let mut data:[u8; BUF_SIZE] = [0; BUF_SIZE];
    _ = match buf_reader.read(&mut data) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)), 
    };

    let mut status = 0;

    let mut http_connection_details = HttpConnectionDetails::new();
    let mut end = false;

    while i < BUF_SIZE && data[i] != 0 && !end{
        match data[i] {
            b' ' => {
                match status {
                    0 => http_connection_details.set_method(buf_string),
                    1 => http_connection_details.set_path(buf_string),
                    _ => (),
                };

                status += 1;

                buf_string = String::new();
            },
            b'\r' => {
                buf_string = String::new();
                end = true;
            },
            _ => buf_string.push(data[i] as char),
        };

        i += 1;

        if i == BUF_SIZE {
            _ = match buf_reader.read(&mut data) {
                Ok(v) => v,
                Err(e) => return Err(Box::new(e)), 
            };

            i = 0;
        }
    }

    let mut key = String::new();

    status = 0;

    while i < BUF_SIZE && data[i] != 0 {
        match status {
            0 => match data[i] {
                b':' => {
                    key = buf_string;

                    status = 1;
                    buf_string = String::new();
                },
                b'\r' => break,
                b'\n' => (),
                _ => buf_string.push(data[i] as char),
            },
            _ => match data[i] {
                b'\r' => {
                    http_connection_details.set_header(key, buf_string);

                    status  = 0;
                    buf_string = String::new();
                    key = String::new();
                },
                _ => {
                    if buf_string.len() > 0 || data[i] != b' ' {
                        buf_string.push(data[i] as char);
                    }
                },
            }
        }

        i += 1;

        if i == BUF_SIZE {
            _ = match buf_reader.read(&mut data) {
                Ok(v) => v,
                Err(e) => return Err(Box::new(e)), 
            };
        }
    }

    while i < BUF_SIZE && data[i] != 0 && (data[i] == b'\r' || data[i] == b'\n') {
        i += 1;

        if i == BUF_SIZE {
            _ = match buf_reader.read(&mut data) {
                Ok(v) => v,
                Err(e) => return Err(Box::new(e)), 
            };
        }
    }

    while i < BUF_SIZE && data[i] != 0 {
        buf_string.push(data[i] as char);

        i += 1;

        if i == BUF_SIZE {
            _ = match buf_reader.read(&mut data) {
                Ok(v) => v,
                Err(e) => return Err(Box::new(e)), 
            };
        }
    }

    http_connection_details.set_data(buf_string);
    
    Ok(http_connection_details)
}
