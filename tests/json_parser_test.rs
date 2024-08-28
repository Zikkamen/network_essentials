#[cfg(test)]
mod tests {
    use network_essentials::string_parser::json_parser;

    #[test]
    fn parse_json_1() {
        let parsed_string = json_parser::parse(&test_json_1());

        println!("{:?}", parsed_string);

        let parsed_string_list = parsed_string.get_list();

        assert_eq!(parsed_string_list.len(), 3);
        assert_eq!(parsed_string_list[0].get_string(), "element_1");
        assert_eq!(parsed_string_list[1].get_string(), "element_2");
        assert_eq!(parsed_string_list[2].get_string(), "element_3 element_4");
    }

    #[test]
    fn parse_json_2() {
        //let parsed_string = json_parser::parse(&test_json_1()).expect("Valid Json");

        //println!("{:?}", parsed_string);

        assert!(false);
    }

    fn test_json_1() -> String {
        "[element_1, element_2, \"element_3 element_4\"]".to_string()
    }

    fn test_json_2() -> String {
        "{element_1, element_2, \"element_3 element_4\"}".to_string()
    }
}