use crate::primitive::FIToFIPaymentStatusReportV12;
use crate::validator::Validate;

pub const FUNCTIONALITY: u8 = 2;
pub const VARIANT: u8 = 1;
pub const VERSION: u8 = 12;
pub const BUSINESS_PROCESS: &'static str = "pacs";
pub const NAMESPACE: &'static str = "urn:iso:std:iso:20022:tech:xsd:pacs.002.001.12";

pub fn namespace() -> String {
    NAMESPACE.to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FIToFIPmtStsRpt")]
    pub inner: FIToFIPaymentStatusReportV12<A>,
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
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.002.001.12.xml").expect("Unable to read file");
        let doc = Document::<Dmkr>::from_xml(file.as_str());
        assert!(doc.is_ok())
    }
}