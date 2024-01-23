use crate::primitive::{
    BusinessApplicationHeader1, BusinessMessagePriorityCode, CopyDuplicate1Code,
    ISONormalisedDateTime, Max35Text, Party9Choice, SignatureEnvelope, UnicodeChartsCode,
    YesNoIndicator,
};
use crate::validator::Validate;

pub const FUNCTIONALITY: u8 = 1;
pub const VARIANT: u8 = 1;
pub const VERSION: u8 = 1;
pub const BUSINESS_PROCESS: &'static str = "head";
pub const NAMESPACE: &'static str = "urn:iso:std:iso:20022:tech:xsd:head.001.001.01";

pub fn namespace() -> String {
    NAMESPACE.to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct AppHdr<
    Signature: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    pub value: BusinessApplicationHeaderV01<Signature>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename = "AppHdr")]
pub struct BusinessApplicationHeaderV01<
    Signature: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub char_set: Option<UnicodeChartsCode>,
    #[serde(rename = "Fr")]
    #[validate]
    pub fr: Party9Choice,
    #[serde(rename = "To")]
    #[validate]
    pub to: Party9Choice,
    #[serde(rename = "BizMsgIdr")]
    #[validate]
    pub biz_msg_idr: Max35Text,
    #[serde(rename = "MsgDefIdr")]
    #[validate]
    pub msg_def_idr: Max35Text,
    #[serde(rename = "BizSvc", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub biz_svc: Option<Max35Text>,
    #[serde(rename = "CreDt")]
    #[validate]
    pub cre_dt: ISONormalisedDateTime,
    #[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<CopyDuplicate1Code>,
    #[serde(rename = "PssblDplct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pssbl_dplct: Option<YesNoIndicator>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prty: Option<BusinessMessagePriorityCode>,
    #[serde(rename = "Sgntr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sgntr: Option<SignatureEnvelope<Signature>>,
    #[serde(rename = "Rltd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rltd: Option<BusinessApplicationHeader1<Signature>>,
}
