#[cfg(test)]
mod tests {
    use network_essentials::string_parser::xml_parser;

    #[test]
    fn parse_xml_1() {
        let parsed_string = xml_parser::parse(&test_xml_1()).expect("Valid Xml");

        let content_1 = parsed_string.get_from_map("group_1").get_string();

        assert_eq!(content_1, "value_1");
    }

    #[test]
    fn parse_xml_2() {
        let parsed_string = xml_parser::parse(&test_xml_2()).expect("Valid Xml");

        let content_1 =  parsed_string.get_from_map("group_1").get_string();

        assert_eq!(content_1, "value_1");

        let content_2 = parsed_string
            .get_from_map("group_2")
            .get_from_map("group_2.1")
            .get_string();

        assert_eq!(content_2, "value_2");
    }

    fn test_xml_1() -> String {
        "<group_1>
            value_1
        </group_1>".to_string()
    }

    fn test_xml_2() -> String {
        "<group_1>
            value_1
        </group_1>
        <group_2>
            <group_2.1>
                value_2
            </group_2.1>
        </group_2>".to_string()
    }
}