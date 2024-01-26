use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OriginalTransactionReference35 {
    #[serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intr_bk_sttlm_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub amt: Option<super::AmountType4Choice>,
    #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub reqd_colltn_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub reqd_exctn_dt: Option<super::DateAndDateTime2Choice>,
    #[serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_schme_id: Option<super::PartyIdentification135>,
    #[serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sttlm_inf: Option<super::SettlementInstruction11>,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation27>,
    #[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
    pub pmt_mtd: Option<super::super::simple_type::PaymentMethod4Code>,
    #[serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub mndt_rltd_inf: Option<super::MandateRelatedData2Choice>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rmt_inf: Option<super::RemittanceInformation21>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ultmt_dbtr: Option<super::Party40Choice>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr: Option<super::Party40Choice>,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr: Option<super::Party40Choice>,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ultmt_cdtr: Option<super::Party40Choice>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub purp: Option<super::Purpose2Choice>,
}