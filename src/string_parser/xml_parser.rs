use std::error;

use crate::string_parser::model_classes::ParsedString;

pub fn parse(s: &String) -> Result<ParsedString, Box<dyn error::Error + 'static>> {
    let raw_data: Vec<char> = s.chars().collect();
    let n: usize = raw_data.len();

    let mut tmp: String = String::new();
    let mut ps_stack = vec![ParsedString::new()];

    let mut entry_desc_stack: Vec<String> = Vec::new();
    let mut entry_stack: Vec<String> = Vec::new();

    /*
        Status 0: Parse contents
        Status 1: Parse content in <...>
        Status 2: Parse conten in </...>
    */
    let mut parse_status: i32 = 0;
    let mut current_line: u32 = 0;

    let mut i: usize = 0;

    while i < n {
        if raw_data[i] == '<' {
            if i == n-1 { 
                panic!("There is an open < at the last position"); 
            }

            if entry_desc_stack.len() > 0 {
                entry_stack.push(tmp.to_owned());
            }

            tmp = String::new();

            if raw_data[i+1] != '/' { //save because of check before
                parse_status = 1;
                i += 1;
            } else {
                if entry_desc_stack.len() == 0 { 
                    panic!("Found closing line without the corresponding object at line: {}", current_line);
                }

                parse_status = 2;
                i += 2;
            }

            continue;
        }

        if raw_data[i] == '>' {
            match parse_status {
                1 => {
                    ps_stack.push(ParsedString::new());
                    entry_desc_stack.push(tmp.to_owned());
                },
                2 => {
                    match entry_desc_stack.pop() {
                        Some(v) => {
                            if tmp != v { 
                                panic!("Closing line doesn't corespond to open line: {} {} at line: {}", v, tmp, current_line); 
                            }

                            match entry_stack.pop() {
                                Some(p) => {
                                    if p.len() > 0 {
                                        ps_stack.last_mut().unwrap().set_string(p);
                                    }
                                },
                                None => panic!("Couldn't find an entry for open line {}", v),
                            }

                            match ps_stack.pop() {
                                Some(p) => ps_stack.last_mut().unwrap().add_to_hashmap(v, p),
                                None => panic!("Error: Credentials Stack is empty"),
                            }
                        
                        },
                        None => panic!("Found closing line without the corresponding object at line: {}", current_line),
                    };
                },
                _ => panic!("Found an > without an open < at line: {}", current_line),
            };

            tmp = String::new();
            parse_status = 0;
            i += 1;
            
            continue;
        }

        if raw_data[i] == '\n' { 
            current_line += 1; 
        }

        if raw_data[i] != '\n' && raw_data[i] != '\r' && raw_data[i] != ' ' {
            tmp.push(raw_data[i]);
        }

        i += 1;
    }

    Ok(ps_stack.last().unwrap().clone())
}