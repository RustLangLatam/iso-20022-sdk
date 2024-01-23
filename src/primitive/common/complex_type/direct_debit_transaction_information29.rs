use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DirectDebitTransactionInformation29<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "PmtId")]
    #[validate]
    pub pmt_id: super::PaymentIdentification13,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation27>,
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
    #[serde(rename = "InstdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "XchgRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<super::super::simple_type::BaseOneRate>,
    #[serde(rename = "ChrgBr")]
    pub chrg_br: super::super::simple_type::ChargeBearerType1Code,
    #[serde(rename = "ChrgsInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub chrgs_inf: Vec<super::Charges7>,
    #[serde(rename = "ReqdColltnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reqd_colltn_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "DrctDbtTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drct_dbt_tx: Option<super::DirectDebitTransaction11>,
    #[serde(rename = "Cdtr")]
    #[validate]
    pub cdtr: super::PartyIdentification135,
    #[serde(rename = "CdtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "CdtrAgt")]
    #[validate]
    pub cdtr_agt: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "CdtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<super::PartyIdentification135>,
    #[serde(rename = "InitgPty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<super::PartyIdentification135>,
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
    #[serde(rename = "Dbtr")]
    #[validate]
    pub dbtr: super::PartyIdentification135,
    #[serde(rename = "DbtrAcct")]
    #[validate]
    pub dbtr_acct: super::CashAccount40,
    #[serde(rename = "DbtrAgt")]
    #[validate]
    pub dbtr_agt: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<super::PartyIdentification135>,
    #[serde(rename = "Purp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purp: Option<super::Purpose2Choice>,
    #[serde(rename = "RgltryRptg")]
    #[validate(length(min = 0, max = 10))]
    #[serde(default)]
    pub rgltry_rptg: Vec<super::RegulatoryReporting3>,
    #[serde(rename = "RltdRmtInf")]
    #[validate(length(min = 0, max = 10))]
    #[serde(default)]
    pub rltd_rmt_inf: Vec<super::RemittanceLocation7>,
    #[serde(rename = "RmtInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<super::RemittanceInformation21>,
    #[serde(rename = "SplmtryData")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}