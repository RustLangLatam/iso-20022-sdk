use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OriginalGroupHeader16 {
    #[serde(rename = "OrgnlMsgId")]
    #[validate]
    pub orgnl_msg_id: super::super::simple_type::Max35Text,
    #[serde(rename = "OrgnlMsgNmId")]
    #[validate]
    pub orgnl_msg_nm_id: super::super::simple_type::Max35Text,
    #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_cre_dt_tm: Option<super::super::simple_type::ISODateTime>,
    #[serde(default, rename = "RvslRsnInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub rvsl_rsn_inf: Vec<super::PaymentReversalReason9>,
}