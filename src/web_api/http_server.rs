use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    error::Error,
    fmt,
};

use crate::web_api::api_register::ApiRegister;
use crate::web_api::api_register::HttpConnectionDetails;

#[derive(Debug)]
struct HttpParsingError(String);

impl fmt::Display for HttpParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for HttpParsingError {}

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
                                    let response = api_register_clone.handle_http_request(hm);

                                    match stream.write_all(response.as_bytes()) {
                                        Ok(v) => v,
                                        Err(_) => continue,
                                    };
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

fn handle_connection(mut stream: &mut TcpStream) -> Result<HttpConnectionDetails,  HttpParsingError> {
    let mut buf_arr = [0; 8192];
    let mut n = match stream.read(&mut buf_arr) {
        Ok(v) => v,
        Err(_) => 0,
    };

    let mut buf_str = String::new();
    let mut key = String::new();
    let mut i:usize = 0;

    let mut http_connection_details = HttpConnectionDetails::new();
    let mut status = 0;

    while i < n {
        match buf_arr[i] {
            b' ' => {
                match status {
                    0 => http_connection_details.set_method(buf_str),
                    1 => match validate_path(&buf_str) {
                        true => http_connection_details.set_path(buf_str),
                        false => return Err(HttpParsingError("Forbidden Character in path".to_string())),
                    },
                    _ => (),
                };

                status += 1;

                buf_str = String::new();
            },
            b'\r' => {
                buf_str = String::new();
                i += 1;

                break;
            },
            _ => buf_str.push(buf_arr[i] as char),
        };

        i += 1;

        if i == n {
            n = match stream.read(&mut buf_arr) {
                Ok(v) => v,
                Err(_) => 0,
            };

            i = 0;
        }
    }

    status = 0;

    while i < n {
        match status {
            0 => match buf_arr[i] {
                b':' => {
                    key = buf_str;

                    status = 1;
                    buf_str = String::new();
                },
                b'\r' => break,
                b'\n' => (),
                _ => buf_str.push(buf_arr[i] as char),
            },
            _ => match buf_arr[i] {
                b'\r' => {
                    http_connection_details.set_header(key, buf_str);

                    status  = 0;
                    buf_str = String::new();
                    key = String::new();
                },
                _ => {
                    if buf_str.len() > 0 || buf_arr[i] != b' ' {
                        buf_str.push(buf_arr[i] as char);
                    }
                },
            }
        }

        i += 1;

        if i == n {
            n = match stream.read(&mut buf_arr) {
                Ok(v) => v,
                Err(_) => 0,
            };

            i = 0;
        }
    }

    while i < n && (buf_arr[i] == b'\r' || buf_arr[i] == b'\n') {
        i += 1;

        if i == n {
            n = match stream.read(&mut buf_arr) {
                Ok(v) => v,
                Err(_) => 0,
            };

            i = 0;
        }
    }

    while i < n {
        buf_str.push(buf_arr[i] as char);

        i += 1;

        if i == n {
            n = match stream.read(&mut buf_arr) {
                Ok(v) => v,
                Err(e) => {
                    println!("{e}");

                    0
                },
            };

            i = 0;
        }
    }

    http_connection_details.set_data(buf_str);
    
    Ok(http_connection_details)
}

fn validate_path(path: &String) -> bool {
    let chars:&[u8] = path.as_bytes();
    let n = chars.len();

    for i in 1..n {
        if chars[i] == b'.' && chars[i-1] == b'.' {
            return false;
        }
    }

    for i in 0..n {
        if chars[i] == b'<' || chars[i] == b'>' {
            return false;
        }
    }

    true
}