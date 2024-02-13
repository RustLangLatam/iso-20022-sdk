use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditTransferTransaction52 {
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ultmt_dbtr: Option<super::PartyIdentification135>,
    #[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub initg_pty: Option<super::PartyIdentification135>,
    #[serde(rename = "Dbtr")]
    #[validate]
    pub dbtr: super::PartyIdentification135,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "DbtrAgt")]
    #[validate]
    pub dbtr_agt: super::BranchAndFinancialInstitutionIdentification6,
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
    #[serde(rename = "CdtrAgt")]
    #[validate]
    pub cdtr_agt: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "Cdtr")]
    #[validate]
    pub cdtr: super::PartyIdentification135,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ultmt_cdtr: Option<super::PartyIdentification135>,
    #[serde(
        default,
        rename = "InstrForCdtrAgt",
        skip_serializing_if = "<[_]>::is_empty"
    )]
    #[validate]
    pub instr_for_cdtr_agt: Vec<super::InstructionForCreditorAgent3>,
    #[serde(
        default,
        rename = "InstrForNxtAgt",
        skip_serializing_if = "<[_]>::is_empty"
    )]
    #[validate]
    pub instr_for_nxt_agt: Vec<super::InstructionForNextAgent1>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tax: Option<super::TaxInformation10>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rmt_inf: Option<super::RemittanceInformation21>,
    #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
}
