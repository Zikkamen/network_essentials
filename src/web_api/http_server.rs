use std::{
    collections::{HashMap},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    error,
    sync::{Arc, RwLock},
};

use crate::web_api::api_register::ApiRegister;

pub struct HttpServer {
    address: String,
    api_register: Arc<RwLock<ApiRegister>>,
}

impl HttpServer {
    pub fn new(address: &str, main_api_register: ApiRegister) -> Self {
        HttpServer{ 
            address: address.to_string(),
            api_register: Arc::new(RwLock::new(main_api_register)),
        }
    }

    pub fn start_listening(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        for stream in listener.incoming() {
            let api_register_clone = Arc::clone(&self.api_register);

            thread::spawn(move || {
                match stream {
                    Ok(mut v) => match handle_connection(&mut v) {
                        Ok(hm) => {
                            println!("Handling Connection");
                            api_register_clone.read().unwrap().handle_http_request(hm, v);
                        },
                        Err(e) => println!("Error handling incoming request {}", e),
                    },
                    Err(e) => println!("Error handling incoming request {}", e),
                };
            });
        }
    }
}

fn handle_connection(mut stream: &mut TcpStream) -> Result<HashMap<String, String>,  Box<dyn error::Error + 'static>> {
    let buf_reader = BufReader::new(&mut stream);
    let mut http_request:HashMap<String, String> = HashMap::new();


    for result in buf_reader.lines() {
        let line:String = match result {
            Ok(v) => v,
            Err(_e) => continue,
        };

        let parameters = split_string_into_pairs(&line);
        http_request.insert(parameters.0, parameters.1);
    }
    
    Ok(http_request)
}

fn split_string_into_pairs(s: &String) -> (String, String) {
    let n: usize = s.len();

    if n == 0 { 
        return (String::new(), String::new()); 
    }

    let sep_pos = match s.find(':') {
        Some(v) => v,
        None => return ("HEAD_REQUEST:".to_string(), s.clone()), 
    };

    let char_array = s.chars();
    
    (
        char_array.clone().take(sep_pos).collect(), 
        char_array.clone().skip(sep_pos+1).take(n-sep_pos).collect()
    )
}