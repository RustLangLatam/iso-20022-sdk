use crate::validator::Validate;

pub const FUNCTIONALITY: u8 = 1;
pub const VARIANT: u8 = 1;
pub const VERSION: u8 = 2;
pub const BUSINESS_PROCESS: &'static str = "head";
pub const NAMESPACE: &'static str = "urn:iso:std:iso:20022:tech:xsd:head.001.001.02";

pub fn namespace() -> String {
    NAMESPACE.to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[serde(transparent)]
pub struct AppHdr<
    Signature: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    pub value: BusinessApplicationHeaderV02<Signature>,
}


#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename = "AppHdr")]
pub struct BusinessApplicationHeaderV02<Signature: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub char_set: Option<crate::primitive::UnicodeChartsCode>,
    #[serde(rename = "Fr")]
    #[validate]
    pub fr: crate::primitive::Party44Choice,
    #[serde(rename = "To")]
    #[validate]
    pub to: crate::primitive::Party44Choice,
    #[serde(rename = "BizMsgIdr")]
    #[validate]
    pub biz_msg_idr: crate::primitive::Max35Text,
    #[serde(rename = "MsgDefIdr")]
    #[validate]
    pub msg_def_idr: crate::primitive::Max35Text,
    #[serde(rename = "BizSvc", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub biz_svc: Option<crate::primitive::Max35Text>,
    #[serde(rename = "MktPrctc", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub mkt_prctc: Option<crate::primitive::ImplementationSpecification1>,
    #[serde(rename = "CreDt")]
    #[validate]
    pub cre_dt: crate::primitive::ISODateTime,
    #[serde(rename = "BizPrcgDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub biz_prcg_dt: Option<crate::primitive::ISODateTime>,
    #[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<crate::primitive::CopyDuplicate1Code>,
    #[serde(rename = "PssblDplct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pssbl_dplct: Option<crate::primitive::YesNoIndicator>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prty: Option<crate::primitive::BusinessMessagePriorityCode>,
    #[serde(rename = "Sgntr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sgntr: Option<crate::primitive::SignatureEnvelope<Signature>>,
    #[serde(rename = "Rltd")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub rltd: Vec<crate::primitive::BusinessApplicationHeader5<Signature>>,
}