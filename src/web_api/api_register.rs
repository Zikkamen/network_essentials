use std::{
    collections::{HashMap},
    net::{TcpStream},
};

type FnType = fn(HttpConnectionDetails, TcpStream);

#[derive(Debug)]
pub struct HttpConnectionDetails {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    data: String,
}

impl HttpConnectionDetails {
    pub fn new() -> Self {
        HttpConnectionDetails{
            method: String::new(),
            path: String::new(),
            headers: HashMap::new(),
            data: String::new(),
        }
    }

    pub fn set_method(&mut self, method: String) {
        match &method[..] {
            "GET" | "PUT" | "POST" | "PATCH" | "DELETE" | "OPTIONS" | "HEAD" => self.method = method,
            _ => panic!("Can't recgonize Http Method {method}"),
        };
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    pub fn get_method(&self) -> String {
        self.method.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers.get(key).cloned()
    }
}

#[derive(Clone)]
pub struct ApiRegister {
    default_func: FnType,
    path_map: HashMap<(String, String), FnType>,
    prefix_map: HashMap<String, Self>,
}

impl ApiRegister {
    pub fn new(default_func: FnType) -> Self {
        ApiRegister{ default_func: default_func, path_map:HashMap::new(), prefix_map:HashMap::new() }
    }

    pub fn handle_http_request(&self, mut http_request: HttpConnectionDetails, stream: TcpStream) {
        let method = http_request.get_method();
        let path = http_request.get_path();

        let mut prefix = "/".to_string();
        let mut suffix = "/".to_string();

        let mut found_prefix = false;
        let mut first = true;

        for c in path.chars() {
            if first {
                first = false;
                continue;
            }

            match found_prefix {
                true => suffix.push(c),
                false => match c {
                    '/' => {
                        found_prefix = true;
                    },
                    _ => prefix.push(c),
                }
            }
        }

        match self.prefix_map.get(&prefix) {
            Some(v) => {
                http_request.set_path(suffix);
                v.handle_http_request(http_request, stream);
                return;
            },
            None => (),
        }

        let func: FnType = match self.path_map.get(&(method, path)) {
            Some(v) => *v,
            None => self.default_func,
        };

        func(http_request, stream);
    }

    pub fn register_function(&mut self, method: &str, path: &str, function: FnType) {
        self.path_map.insert((method.to_string(), path.to_string()), function);
    }

    pub fn register_prefix(&mut self, prefix: &str, api_register: ApiRegister) {
        self.prefix_map.insert(prefix.to_string(), api_register);
    }
}
