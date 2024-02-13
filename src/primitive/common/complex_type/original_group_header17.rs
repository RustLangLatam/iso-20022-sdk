use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OriginalGroupHeader17 {
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
    #[serde(rename = "GrpSts", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub grp_sts: Option<super::super::simple_type::ExternalPaymentGroupStatus1Code>,
    #[serde(default, rename = "StsRsnInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub sts_rsn_inf: Vec<super::StatusReasonInformation12>,
    #[serde(
        default,
        rename = "NbOfTxsPerSts",
        skip_serializing_if = "<[_]>::is_empty"
    )]
    #[validate]
    pub nb_of_txs_per_sts: Vec<super::NumberOfTransactionsPerStatus5>,
}
