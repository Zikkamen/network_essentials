#[cfg(test)]
mod tests {
    use network_essentials::string_parser::json_parser;

    #[test]
    fn parse_json_1() {
        let parsed_string = json_parser::parse(&test_json_1());
        let parsed_string_list = parsed_string.get_list();

        assert_eq!(parsed_string_list.len(), 3);
        assert_eq!(parsed_string_list[0].get_string(), "element_1");
        assert_eq!(parsed_string_list[1].get_string(), "element_2");
        assert_eq!(parsed_string_list[2].get_string(), "element_3 element_4");
    }

    #[test]
    fn parse_json_2() {
        let parsed_string = json_parser::parse(&test_json_2());

        let value_1 = parsed_string.get_from_map("key_1");
        assert_eq!(value_1.get_string(), "1");

        let value_2 = parsed_string.get_from_map("key_2");
        let value_2_list = value_2.get_list();

        assert_eq!(value_2_list.len(), 3);
        assert_eq!(value_2_list[0].get_string(), "1");
        assert_eq!(value_2_list[1].get_string(), "2");
        assert_eq!(value_2_list[2].get_string(), "3");

        let value_3 = parsed_string.get_from_map("key_3");
        let value_4 = value_3.get_from_map("key_4");

        assert_eq!(value_4.get_string(), "4");
    }

    fn test_json_1() -> String {
        "[element_1, element_2, \"element_3 element_4\"]".to_string()
    }

    fn test_json_2() -> String {
        "{\"key_1\": 1, \"key_2\": [1, 2, 3], \"key_3\": {\"key_4\": 4}}".to_string()
    }
}