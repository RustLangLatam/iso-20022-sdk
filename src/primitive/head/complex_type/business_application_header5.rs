use ::validator::Validate;

use crate::primitive::{BusinessMessagePriorityCode, CopyDuplicate1Code, ISODateTime, Max35Text, UnicodeChartsCode, YesNoIndicator};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct BusinessApplicationHeader5<Signature: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub char_set: Option<UnicodeChartsCode>,
    #[serde(rename = "Fr")]
    #[validate]
    pub fr: super::Party44Choice,
    #[serde(rename = "To")]
    #[validate]
    pub to: super::Party44Choice,
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
    pub cre_dt: ISODateTime,
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
    pub sgntr: Option<super::SignatureEnvelope<Signature>>,
}