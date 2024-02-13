use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct SettlementInstruction11 {
    #[serde(rename = "SttlmMtd")]
    pub sttlm_mtd: super::super::simple_type::SettlementMethod1Code,
    #[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sttlm_acct: Option<super::CashAccount40>,
    #[serde(rename = "ClrSys", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub clr_sys: Option<super::ClearingSystemIdentification3Choice>,
    #[serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instg_rmbrsmnt_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(
        rename = "InstgRmbrsmntAgtAcct",
        skip_serializing_if = "Option::is_none"
    )]
    #[validate]
    pub instg_rmbrsmnt_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instd_rmbrsmnt_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(
        rename = "InstdRmbrsmntAgtAcct",
        skip_serializing_if = "Option::is_none"
    )]
    #[validate]
    pub instd_rmbrsmnt_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub thrd_rmbrsmnt_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(
        rename = "ThrdRmbrsmntAgtAcct",
        skip_serializing_if = "Option::is_none"
    )]
    #[validate]
    pub thrd_rmbrsmnt_agt_acct: Option<super::CashAccount40>,
}
