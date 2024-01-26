use crate::primitive::FinancialInstitutionCreditTransferV10;
use crate::validator::Validate;

pub const FUNCTIONALITY: u8 = 9;
pub const VARIANT: u8 = 1;
pub const VERSION: u8 = 10;
pub const BUSINESS_PROCESS: &'static str = "pacs";
pub const NAMESPACE: &'static str = "urn:iso:std:iso:20022:tech:xsd:pacs.009.001.10";

pub fn namespace() -> String {
    NAMESPACE.to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FICdtTrf")]
    pub inner: FinancialInstitutionCreditTransferV10<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}