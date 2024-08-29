use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ParsedString {
    ps_string: Option<String>,
    ps_hashmap: Option<HashMap<String, ParsedString>>,
    ps_list: Option<Vec<ParsedString>>,
}

impl ParsedString {
    pub fn new() -> Self {
        ParsedString{ 
            ps_string: None,
            ps_hashmap: None,
            ps_list: None,
        }
    }

    pub fn add_to_hashmap(&mut self, key: String, ps: ParsedString) {
        if self.ps_hashmap.is_none() {
            self.ps_hashmap = Some(HashMap::new());
        }

        self.ps_hashmap.as_mut().unwrap().insert(key, ps);
    }

    pub fn add_to_list(&mut self, ps: ParsedString) {
        if self.ps_list.is_none() {
            self.ps_list = Some(Vec::new());
        }

        self.ps_list.as_mut().unwrap().push(ps);
    }

    pub fn set_string(&mut self, s: String) {
        self.ps_string = Some(s);
    }

    pub fn get_string(&self) -> String {
        match self.ps_string.clone() {
            Some(v) => v,
            None => panic!("Tried to get String when empty"),
        }
    }

    pub fn get_map(&self) -> &HashMap<String, ParsedString> {
        match &self.ps_hashmap {
            Some(v) => &v,
            None => panic!("Tried to get Hashmap when empty"),
        }
    }

    pub fn get_list(&self) -> &Vec<ParsedString> {
        match &self.ps_list {
            Some(v) => &v,
            None => panic!("Tried to get List when empty"),
        }
    }

    pub fn get_from_map(&self, key: &str) -> &ParsedString {
        match &self.get_map().get(key) {
            Some(v) => &v,
            None => panic!("Couldn't find key {} in map", key),
        }
    }
}