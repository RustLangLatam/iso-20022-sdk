use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OriginalGroupInformation27 {
    #[serde(rename = "OrgnlMsgId")]
    #[validate]
    pub orgnl_msg_id: super::super::simple_type::Max35Text,
    #[serde(rename = "OrgnlMsgNmId")]
    #[validate]
    pub orgnl_msg_nm_id: super::super::simple_type::Max35Text,
    #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_cre_dt_tm: Option<super::super::simple_type::ISODateTime>,
    #[serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_nb_of_txs: Option<super::super::simple_type::Max15NumericText>,
    #[serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_ctrl_sum: Option<super::super::simple_type::DecimalNumber>,
}
