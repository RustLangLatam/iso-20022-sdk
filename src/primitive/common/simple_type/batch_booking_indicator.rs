#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct BatchBookingIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}

#[cfg(test)]
mod tests {
    use crate::utils::XmlExt;

    use super::*;

    #[test]
    fn test_parse_xml_document() {
        let xml = "<BatchBookingIndicator>true</BatchBookingIndicator>";

        let value = BatchBookingIndicator::from_xml(xml);
        println!("{:?}", value);

        // assert!(doc.is_ok());
        // assert!(doc.unwrap().validate().is_ok());
        assert!(false)
    }
}
