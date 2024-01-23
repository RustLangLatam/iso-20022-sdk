use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditTransferTransaction56<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "PmtId")]
    #[validate]
    pub pmt_id: super::PaymentIdentification13,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation28>,
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
    #[serde(rename = "SttlmTmReq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_req: Option<super::SettlementTimeRequest2>,
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
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Dbtr")]
    #[validate]
    pub dbtr: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "DbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "DbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<super::CashAccount40>,
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
    #[serde(rename = "InstrForNxtAgt")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub instr_for_nxt_agt: Vec<super::InstructionForNextAgent1>,
    #[serde(rename = "Purp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purp: Option<super::Purpose2Choice>,
    #[serde(rename = "RmtInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<super::RemittanceInformation2>,
    #[serde(rename = "UndrlygCstmrCdtTrf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undrlyg_cstmr_cdt_trf: Option<super::CreditTransferTransaction52>,
    #[serde(rename = "SplmtryData")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}