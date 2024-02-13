#[cfg(test)]
mod tests {
    use crate::crypto::ecdsa::EcdsaSignature;
    use crate::documents::Dmkr;
    use crate::models::head::AppHdr;

    #[test]
    fn test_parse_xml_head_001_001_01() {
        let file = std::fs::read_to_string("test/resources/head/head.001.001.01-sgntr-ecdsa.xml")
            .expect("Unable to read file");

        let doc = AppHdr::<EcdsaSignature>::from_xml(file.as_str());

        assert!(doc.clone().is_ok_and(|v| v.is_head_001_001_01()));
    }

    #[test]
    fn test_parse_xml_head_001_001_02() {
        let file = std::fs::read_to_string("test/resources/head/head.001.001.02.xml")
            .expect("Unable to read file");

        let doc = AppHdr::<Dmkr>::from_xml(file.as_str());
        assert!(doc.clone().is_ok_and(|v| v.is_head_001_001_02()));
        assert_eq!(file, doc.unwrap().to_xml().unwrap());
        assert!(false)
    }
}
