use ::validator::Validate;

use crate::primitive::Max35Text;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct BusinessApplicationHeader1<
    Signature: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub char_set: Option<super::super::simple_type::UnicodeChartsCode>,
    #[serde(rename = "Fr")]
    #[validate]
    pub fr: super::Party9Choice,
    #[serde(rename = "To")]
    #[validate]
    pub to: super::Party9Choice,
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
    pub cre_dt: super::super::simple_type::ISONormalisedDateTime,
    #[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<super::super::simple_type::CopyDuplicate1Code>,
    #[serde(rename = "PssblDplct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pssbl_dplct: Option<super::super::simple_type::YesNoIndicator>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prty: Option<super::super::simple_type::BusinessMessagePriorityCode>,
    #[serde(rename = "Sgntr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sgntr: Option<super::SignatureEnvelope<Signature>>,
}
