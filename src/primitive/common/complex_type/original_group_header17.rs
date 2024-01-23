use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OriginalGroupHeader17 {
    #[serde(rename = "OrgnlMsgId")]
    #[validate]
    pub orgnl_msg_id: super::super::simple_type::Max35Text,
    #[serde(rename = "OrgnlMsgNmId")]
    #[validate]
    pub orgnl_msg_nm_id: super::super::simple_type::Max35Text,
    #[serde(rename = "OrgnlCreDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<super::super::simple_type::ISODateTime>,
    #[serde(rename = "OrgnlNbOfTxs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_nb_of_txs: Option<super::super::simple_type::Max15NumericText>,
    #[serde(rename = "OrgnlCtrlSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_ctrl_sum: Option<super::super::simple_type::DecimalNumber>,
    #[serde(rename = "GrpSts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_sts: Option<super::super::simple_type::ExternalPaymentGroupStatus1Code>,
    #[serde(rename = "StsRsnInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub sts_rsn_inf: Vec<super::StatusReasonInformation12>,
    #[serde(rename = "NbOfTxsPerSts")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub nb_of_txs_per_sts: Vec<super::NumberOfTransactionsPerStatus5>,
}