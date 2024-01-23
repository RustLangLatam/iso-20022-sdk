use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DirectDebitTransactionInformation27 {
    #[serde(rename = "PmtId")]
    #[validate]
    pub pmt_id: super::PaymentIdentification13,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation28>,
    #[serde(rename = "IntrBkSttlmAmt")]
    #[validate]
    pub intr_bk_sttlm_amt: super::ActiveCurrencyAndAmount,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "SttlmPrty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_prty: Option<super::super::simple_type::Priority3Code>,
    #[serde(rename = "SttlmTmIndctn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_indctn: Option<super::SettlementDateTimeIndication1>,
    #[serde(rename = "SttlmTmReq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_req: Option<super::SettlementTimeRequest2>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Dbtr")]
    #[validate]
    pub dbtr: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "DbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "DbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "InstrForDbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_for_dbtr_agt: Option<super::super::simple_type::Max210Text>,
    #[serde(rename = "Purp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purp: Option<super::Purpose2Choice>,
    #[serde(rename = "RmtInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<super::RemittanceInformation2>,
}