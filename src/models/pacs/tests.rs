#[cfg(test)]
mod tests {
    use crate::models::pacs::pacs_002_001_12;
    use crate::models::pacs::pacs_003_001_09;
    use crate::models::pacs::pacs_004_001_11;
    use crate::models::pacs::pacs_007_001_11;
    use crate::models::pacs::pacs_008_001_10;
    use crate::models::pacs::pacs_009_001_10;
    use crate::models::pacs::pacs_010_001_05;
    use crate::models::pacs::pacs_028_001_05;
    use crate::primitive::Dmkr;
    use crate::utils::XmlExt;

    #[test]
    fn test_parse_xml_pacs_002_001_12() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.002.001.12.xml")
            .expect("Unable to read file");
        let doc = pacs_002_001_12::Document::<Dmkr>::from_xml(file.as_str());
        assert!(doc.is_ok())
    }

    #[test]
    fn test_parse_xml_pacs_003_001_09() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.003.001.9.xml")
            .expect("Unable to read file");
        let doc = pacs_003_001_09::Document::<Dmkr>::from_xml(file.as_str());
        assert!(doc.is_ok())
    }

    #[test]
    fn test_parse_xml_pacs_004_001_11() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.004.001.11.xml")
            .expect("Unable to read file");
        let doc = pacs_004_001_11::Document::<Dmkr>::from_xml(file.as_str());
        assert!(doc.is_ok())
    }

    #[test]
    fn test_parse_xml_pacs_007_001_11() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.007.001.11.xml")
            .expect("Unable to read file");
        let doc = pacs_007_001_11::Document::<Dmkr>::from_xml(file.as_str());
        assert!(doc.is_ok())
    }

    #[test]
    fn test_parse_xml_pacs_008_001_10() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.008.001.10.xml")
            .expect("Unable to read file");
        let doc = pacs_008_001_10::Document::<Dmkr>::from_xml(file.as_str());

        assert!(doc.is_ok());
        assert_eq!(file, doc.unwrap().to_ident_xml().unwrap());
        assert!(false);
    }

    #[test]
    fn test_parse_xml_pacs_009_001_10() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.009.001.10.xml")
            .expect("Unable to read file");
        let doc = pacs_009_001_10::Document::<Dmkr>::from_xml(file.as_str());
        assert!(doc.is_ok())
    }

    #[test]
    fn test_parse_xml_pacs_010_001_05() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.010.001.05.xml")
            .expect("Unable to read file");
        let doc = pacs_010_001_05::Document::<Dmkr>::from_xml(file.as_str());
        assert!(doc.is_ok())
    }

    #[test]
    fn test_parse_xml_pacs_028_001_05() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.028.001.05.xml")
            .expect("Unable to read file");
        let doc = pacs_028_001_05::Document::<Dmkr>::from_xml(file.as_str());
        assert!(doc.is_ok())
    }
}
