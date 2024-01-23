use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditTransferTransaction52 {
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<super::PartyIdentification135>,
    #[serde(rename = "InitgPty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<super::PartyIdentification135>,
    #[serde(rename = "Dbtr")]
    #[validate]
    pub dbtr: super::PartyIdentification135,
    #[serde(rename = "DbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "DbtrAgt")]
    #[validate]
    pub dbtr_agt: super::BranchAndFinancialInstitutionIdentification6,
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
    #[validate]
    pub cdtr_agt: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "CdtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "Cdtr")]
    #[validate]
    pub cdtr: super::PartyIdentification135,
    #[serde(rename = "CdtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<super::PartyIdentification135>,
    #[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Vec::is_empty")]
    pub instr_for_cdtr_agt: Vec<super::InstructionForCreditorAgent3>,
    #[serde(rename = "InstrForNxtAgt")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub instr_for_nxt_agt: Vec<super::InstructionForNextAgent1>,
    #[serde(rename = "Tax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<super::TaxInformation10>,
    #[serde(rename = "RmtInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<super::RemittanceInformation21>,
    #[serde(rename = "InstdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
}