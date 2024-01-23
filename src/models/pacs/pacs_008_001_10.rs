use crate::primitive::FIToFICustomerCreditTransferV10;
use crate::validator::Validate;

pub const FUNCTIONALITY: u8 = 8;
pub const VARIANT: u8 = 1;
pub const VERSION: u8 = 10;
pub const BUSINESS_PROCESS: &'static str = "pacs";
pub const NAMESPACE: &'static str = "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.10";

pub fn namespace() -> String {
    NAMESPACE.to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FIToFICstmrCdtTrf")]
    pub inner: FIToFICustomerCreditTransferV10<A>,
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
        let file = std::fs::read_to_string("test/resources/documents/pacs/pacs.008.001.10.xml").expect("Unable to read file");
        let doc = Document::<Dmkr>::from_xml(file.as_str());
        // println!("{:#?}", doc);

        let a = serde_json::to_string_pretty(&doc.unwrap()).unwrap();
        println!("{}", a);
        assert!(false)
    }
}