use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditTransferTransaction53<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "CdtId")]
    #[validate]
    pub cdt_id: super::super::simple_type::Max35Text,
    #[serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub btch_bookg: Option<super::super::simple_type::BatchBookingIndicator>,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation28>,
    #[serde(rename = "TtlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ttl_intr_bk_sttlm_amt: Option<super::ActiveCurrencyAndAmount>,
    #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sttlm_tm_indctn: Option<super::SettlementDateTimeIndication1>,
    #[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instg_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instd_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
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
    pub cdtr: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ultmt_cdtr: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(default, rename = "InstrForCdtrAgt", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub instr_for_cdtr_agt: Vec<super::InstructionForCreditorAgent3>,
    #[serde(rename = "DrctDbtTxInf")]
    #[validate(length(min = 1,))]
    pub drct_dbt_tx_inf: Vec<super::DirectDebitTransactionInformation27>,
    #[serde(default, rename = "SplmtryData", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}