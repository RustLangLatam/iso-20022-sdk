use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TransactionParties10 {
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ultmt_dbtr: Option<super::Party40Choice>,
    #[serde(rename = "Dbtr")]
    #[validate]
    pub dbtr: super::Party40Choice,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub initg_pty: Option<super::Party40Choice>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prvs_instg_agt1: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prvs_instg_agt1_acct: Option<super::CashAccount40>,
    #[serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prvs_instg_agt2: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prvs_instg_agt2_acct: Option<super::CashAccount40>,
    #[serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prvs_instg_agt3: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prvs_instg_agt3_acct: Option<super::CashAccount40>,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intrmy_agt1: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intrmy_agt1_acct: Option<super::CashAccount40>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intrmy_agt2: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intrmy_agt2_acct: Option<super::CashAccount40>,
    #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intrmy_agt3: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intrmy_agt3_acct: Option<super::CashAccount40>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "Cdtr")]
    #[validate]
    pub cdtr: super::Party40Choice,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ultmt_cdtr: Option<super::Party40Choice>,
}
