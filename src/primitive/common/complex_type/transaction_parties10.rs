use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TransactionParties10 {
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<super::Party40Choice>,
    #[serde(rename = "Dbtr")]
    #[validate]
    pub dbtr: super::Party40Choice,
    #[serde(rename = "DbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "InitgPty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<super::Party40Choice>,
    #[serde(rename = "DbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "PrvsInstgAgt1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt1Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1_acct: Option<super::CashAccount40>,
    #[serde(rename = "PrvsInstgAgt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt2Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2_acct: Option<super::CashAccount40>,
    #[serde(rename = "PrvsInstgAgt3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt3Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3_acct: Option<super::CashAccount40>,
    #[serde(rename = "IntrmyAgt1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt1Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1_acct: Option<super::CashAccount40>,
    #[serde(rename = "IntrmyAgt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt2Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2_acct: Option<super::CashAccount40>,
    #[serde(rename = "IntrmyAgt3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt3Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3_acct: Option<super::CashAccount40>,
    #[serde(rename = "CdtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "Cdtr")]
    #[validate]
    pub cdtr: super::Party40Choice,
    #[serde(rename = "CdtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<super::Party40Choice>,
}