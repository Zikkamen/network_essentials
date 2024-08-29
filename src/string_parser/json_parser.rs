use std::error;
use std::collections::HashMap;

use crate::string_parser::model_classes::ParsedString;

pub fn parse(s: &String) -> ParsedString {
    let mut filtered_data: Vec<char> = Vec::new();
    let mut open_string = false;

    for c in s.chars() {
        if c == '"' {
            open_string = !open_string;
        }
        
        if open_string {
            filtered_data.push(c);

            continue;
        }

        match c {
            '\n' | '\r' | '\t' | ' ' => (),
            _ => filtered_data.push(c),
        };
    }

    if filtered_data.len() == 0 {
        panic!("Exptected an non Empty String");
    }

    match filtered_data[0] {
        '[' => return parse_list(1, &filtered_data).1,
        '{' => return parse_map(1, &filtered_data).1,
        _ => (),
    }

    panic!("Couldn't find open {{ or open [");
}

fn parse_list(pos: usize, raw_data: &Vec<char>) -> (usize, ParsedString) {
    let n:usize = raw_data.len();
    let mut parsed_string = ParsedString::new();
    let mut i = pos;

    while i < n {
        match raw_data[i] {
            ']' => return (i+1, parsed_string),
            '}' => panic!("}} was found without corresponding open bracket"),
            _ => { 
                let (npos, parsed_string_new) = match raw_data[i] {
                    '{' => parse_map(i+1, raw_data),
                    '[' => parse_list(i+1, raw_data),
                    _ => parse_string(i, raw_data),
                };

                i = npos;
                parsed_string.add_to_list(parsed_string_new);

                match raw_data[i] {
                    '}' | ']' => (),
                    ',' => i += 1,
                    _ => panic!("Error finding closing statement. Position: {}", i),
                }

                continue;
            },
        };

        i += 1;
    }

    panic!("Couldn't find corresponding ]");
}

fn parse_map(pos: usize, raw_data: &Vec<char>) -> (usize, ParsedString) {
    let n:usize = raw_data.len();
    let mut parsed_string = ParsedString::new();

    let mut i = pos;
    let mut key = String::new();

    while i < n {
        match raw_data[i] {
            '}' => return (i+1, parsed_string),
            ']' => panic!("] was found without corresponding open bracket"),
            '"' => i += 1,
            _ => panic!("Expected to find \". Instead found {}", raw_data[i]),
        };

        while i < n {
            match raw_data[i] {
                '"' => { i += 1; break; },
                _ => key.push(raw_data[i]),
            };

            i += 1;
        }

        if i == n || raw_data[i] != ':' {
            panic!("Expected to find :");
        }

        i += 1;

        let (npos, parsed_string_new) = match raw_data[i] {
            '{' => parse_map(i+1, raw_data),
            '[' => parse_list(i+1, raw_data),
            _ => parse_string(i, raw_data),
        };

        i = npos;
        parsed_string.add_to_hashmap(key, parsed_string_new);

        key = String::new();

        match raw_data[i] {
            '}' | ']' => (),
            ',' => i += 1,
            _ => panic!("Error finding closing statement. Position: {}", i),
        }
    }

    panic!("Couldn't find corresponding }}");
}

fn parse_string(pos: usize, raw_data: &Vec<char>) -> (usize, ParsedString) {
    let n:usize = raw_data.len();
    let mut parsed_string = ParsedString::new();

    let mut tmp:String = String::new();
    let mut open_string = false;

    let mut i = pos;

    while i < n {
        match open_string {
            true => match raw_data[i] {
                '"' => {
                    parsed_string.set_string(tmp);

                    return (i+1, parsed_string);
                },
                _ => tmp.push(raw_data[i]),
            },
            false => match raw_data[i] {
                '"' => {
                    open_string = true;

                    if tmp.len() > 0 {
                        panic!("Syntax Error: Having \" in string: {}", tmp);
                    }
                }
                ',' | '}' | ']' => {
                    parsed_string.set_string(tmp);

                    return (i, parsed_string)
                },
                _ => tmp.push(raw_data[i]),
            },
        };

        i += 1;
    }

    panic!("Couldn't find corresponding \"");
}