mod tests {
    use crate::documents::Document;

    // #[test]
    // fn test_from_namespace() {
    //     let doc = Document::from_namespace("pacs.008.001.10");
    //     assert!(false)
    // }

    #[test]
    fn test_parse_xml_document() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.008.001.10.xml")
            .expect("Unable to read file");
        let doc = Document::from_xml(&file);

        assert!(doc.clone().is_ok_and(|d| d
            .value
            .is_some_and(|v| v.as_pacs().is_some_and(|x| x.is_pacs_008_001_10()))));

        let json = serde_json::to_string_pretty(&doc.unwrap());
        assert!(json.is_ok())
    }
}
