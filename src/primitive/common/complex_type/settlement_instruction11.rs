use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct SettlementInstruction11 {
    #[serde(rename = "SttlmMtd")]
    pub sttlm_mtd: super::super::simple_type::SettlementMethod1Code,
    #[serde(rename = "SttlmAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_acct: Option<super::CashAccount40>,
    #[serde(rename = "ClrSys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys: Option<super::ClearingSystemIdentification3Choice>,
    #[serde(rename = "InstgRmbrsmntAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstgRmbrsmntAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "InstdRmbrsmntAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdRmbrsmntAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "ThrdRmbrsmntAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thrd_rmbrsmnt_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "ThrdRmbrsmntAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thrd_rmbrsmnt_agt_acct: Option<super::CashAccount40>,
}