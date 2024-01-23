use ::validator::Validate;
use crate::primitive::{ISODateTime, Max35Text};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct BusinessApplicationHeader5<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "CharSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_set: Option<super::super::simple_type::UnicodeChartsCode>,
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
    #[serde(rename = "BizSvc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_svc: Option<Max35Text>,
    #[serde(rename = "CreDt")]
    #[validate]
    pub cre_dt: ISODateTime,
    #[serde(rename = "CpyDplct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<super::super::simple_type::CopyDuplicate1Code>,
    #[serde(rename = "PssblDplct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pssbl_dplct: Option<super::super::simple_type::YesNoIndicator>,
    #[serde(rename = "Prty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prty: Option<super::super::simple_type::BusinessMessagePriorityCode>,
    #[serde(rename = "Sgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgntr: Option<super::SignatureEnvelope<A>>,
}