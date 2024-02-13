use crate::primitive::FIToFICustomerDirectDebitV09;
use crate::validator::Validate;

pub const FUNCTIONALITY: u8 = 3;
pub const VARIANT: u8 = 1;
pub const VERSION: u8 = 9;
pub const BUSINESS_PROCESS: &'static str = "pacs";
pub const NAMESPACE: &'static str = "urn:iso:std:iso:20022:tech:xsd:pacs.003.001.09";

pub fn namespace() -> String {
    NAMESPACE.to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FIToFICstmrDrctDbt")]
    pub inner: FIToFICustomerDirectDebitV09<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
