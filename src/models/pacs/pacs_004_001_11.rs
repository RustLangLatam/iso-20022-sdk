use crate::primitive::PaymentReturnV11;
use crate::validator::Validate;

pub const FUNCTIONALITY: u8 = 4;
pub const VARIANT: u8 = 1;
pub const VERSION: u8 = 11;
pub const BUSINESS_PROCESS: &'static str = "pacs";
pub const NAMESPACE: &'static str = "urn:iso:std:iso:20022:tech:xsd:pacs.004.001.11";

pub fn namespace() -> String {
    NAMESPACE.to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "PmtRtr")]
    pub inner: PaymentReturnV11<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Dmkr;
    use crate::utils::XmlExt;

    #[test]
    fn test_parse_xml_document() {
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.004.001.11.xml").expect("Unable to read file");
        let doc = Document::<Dmkr>::from_xml(file.as_str());
        assert!(doc.is_ok())
    }
}