use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DirectDebitTransactionInformation27 {
    #[serde(rename = "PmtId")]
    #[validate]
    pub pmt_id: super::PaymentIdentification13,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation28>,
    #[serde(rename = "IntrBkSttlmAmt")]
    #[validate]
    pub intr_bk_sttlm_amt: super::ActiveCurrencyAndAmount,
    #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none")]
    pub sttlm_prty: Option<super::super::simple_type::Priority3Code>,
    #[serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sttlm_tm_indctn: Option<super::SettlementDateTimeIndication1>,
    #[serde(rename = "SttlmTmReq", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sttlm_tm_req: Option<super::SettlementTimeRequest2>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ultmt_dbtr: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Dbtr")]
    #[validate]
    pub dbtr: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "InstrForDbtrAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instr_for_dbtr_agt: Option<super::super::simple_type::Max210Text>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub purp: Option<super::Purpose2Choice>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rmt_inf: Option<super::RemittanceInformation2>,
}
