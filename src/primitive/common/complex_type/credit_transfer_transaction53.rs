use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditTransferTransaction53<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "CdtId")]
    #[validate]
    pub cdt_id: super::super::simple_type::Max35Text,
    #[serde(rename = "BtchBookg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btch_bookg: Option<super::super::simple_type::BatchBookingIndicator>,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation28>,
    #[serde(rename = "TtlIntrBkSttlmAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_intr_bk_sttlm_amt: Option<super::ActiveCurrencyAndAmount>,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "SttlmTmIndctn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_indctn: Option<super::SettlementDateTimeIndication1>,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
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
    pub cdtr: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "CdtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Vec::is_empty")]
    pub instr_for_cdtr_agt: Vec<super::InstructionForCreditorAgent3>,
    #[serde(rename = "DrctDbtTxInf")]
    #[validate(length(min = 1,))]
    pub drct_dbt_tx_inf: Vec<super::DirectDebitTransactionInformation27>,
    #[serde(rename = "SplmtryData")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}