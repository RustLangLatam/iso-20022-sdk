use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GroupHeader97 {
    #[serde(rename = "MsgId")]
    #[validate]
    pub msg_id: super::super::simple_type::Max35Text,
    #[serde(rename = "CreDtTm")]
    #[validate]
    pub cre_dt_tm: super::super::simple_type::ISODateTime,
    #[serde(rename = "Authstn")]
    #[validate(length(min = 0, max = 2))]
    #[serde(default)]
    pub authstn: Vec<super::Authorisation1Choice>,
    #[serde(rename = "BtchBookg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btch_bookg: Option<super::super::simple_type::BatchBookingIndicator>,
    #[serde(rename = "NbOfTxs")]
    #[validate]
    pub nb_of_txs: super::super::simple_type::Max15NumericText,
    #[serde(rename = "CtrlSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrl_sum: Option<super::super::simple_type::DecimalNumber>,
    #[serde(rename = "GrpRvsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_rvsl: Option<super::super::simple_type::TrueFalseIndicator>,
    #[serde(rename = "TtlRvsdIntrBkSttlmAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_rvsd_intr_bk_sttlm_amt: Option<super::ActiveCurrencyAndAmount>,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "SttlmInf")]
    #[validate]
    pub sttlm_inf: super::SettlementInstruction11,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
}