use crate::primitive::FIToFIPaymentReversalV11;
use crate::validator::Validate;

pub const FUNCTIONALITY: u8 = 7;
pub const VARIANT: u8 = 1;
pub const VERSION: u8 = 11;
pub const BUSINESS_PROCESS: &'static str = "pacs";
pub const NAMESPACE: &'static str = "urn:iso:std:iso:20022:tech:xsd:pacs.007.001.11";

pub fn namespace() -> String {
    NAMESPACE.to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FIToFIPmtRvsl")]
    pub inner: FIToFIPaymentReversalV11<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
