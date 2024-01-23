use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GroupHeader96 {
    #[serde(rename = "MsgId")]
    #[validate]
    pub msg_id: super::super::simple_type::Max35Text,
    #[serde(rename = "CreDtTm")]
    #[validate]
    pub cre_dt_tm: super::super::simple_type::ISODateTime,
    #[serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub btch_bookg: Option<super::super::simple_type::BatchBookingIndicator>,
    #[serde(rename = "NbOfTxs")]
    #[validate]
    pub nb_of_txs: super::super::simple_type::Max15NumericText,
    #[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctrl_sum: Option<super::super::simple_type::DecimalNumber>,
    #[serde(rename = "TtlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ttl_intr_bk_sttlm_amt: Option<super::ActiveCurrencyAndAmount>,
    #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "SttlmInf")]
    #[validate]
    pub sttlm_inf: super::SettlementInstruction11,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation28>,
    #[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instg_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instd_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
}
