use std::error;
use std::collections::HashMap;

use crate::string_parser::model_classes::ParsedString;

pub fn parse(s: &String) -> ParsedString {
    let raw_data: Vec<char> = s.chars().collect();
    let n = raw_data.len();
    let mut i:usize = 0;

    while i < n {
        match raw_data[i] {
            '[' => return parse_list(i+1, &raw_data).1,
            '{' => return parse_map(i+1, &raw_data).1,
            _ => i += 1,
        }
    }

    panic!("Couldn't find open {{ or open [");
}

fn parse_list(pos: usize, raw_data: &Vec<char>) -> (usize, ParsedString) {
    let n:usize = raw_data.len();
    let mut parsed_string = ParsedString::new();
    let mut i = pos;

    while i < n {
        match raw_data[i] {
            '\n' | '\r' | '\t' | ' ' => (),
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

                while i < n {
                    match raw_data[i] {
                        '\n' | '\r' | '\t' | ' ' => (),
                        '}' | ']' => break,
                        ',' => { i += 1; break; },
                        _ => panic!("Error finding closing statement. Position: {}", i),
                    }

                    i += 1;
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
                '\n' | '\r' | '\t' | ' ' => (),
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