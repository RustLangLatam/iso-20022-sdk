#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct YesNoIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}

#[cfg(test)]
mod tests {
    use crate::utils::XmlExt;

    use super::*;

    #[test]
    fn test_parse_xml_document() {
        let xml = "<PssblDplct>true</PssblDplct>";

        let value = YesNoIndicator::from_xml(xml);

        assert!(value.is_ok())
    }
}
