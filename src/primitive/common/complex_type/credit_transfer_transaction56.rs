use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditTransferTransaction56<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "PmtId")]
    #[validate]
    pub pmt_id: super::PaymentIdentification13,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation28>,
    #[serde(rename = "IntrBkSttlmAmt")]
    #[validate]
    pub intr_bk_sttlm_amt: super::ActiveCurrencyAndAmount,
    #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none")]
    pub sttlm_prty: Option<super::super::simple_type::Priority3Code>,
    #[serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sttlm_tm_indctn: Option<super::SettlementDateTimeIndication1>,
    #[serde(rename = "SttlmTmReq", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sttlm_tm_req: Option<super::SettlementTimeRequest2>,
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
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ultmt_dbtr: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Dbtr")]
    #[validate]
    pub dbtr: super::BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_agt_acct: Option<super::CashAccount40>,
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
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub purp: Option<super::Purpose2Choice>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rmt_inf: Option<super::RemittanceInformation2>,
    #[serde(rename = "UndrlygCstmrCdtTrf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub undrlyg_cstmr_cdt_trf: Option<super::CreditTransferTransaction52>,
    #[serde(
        default,
        rename = "SplmtryData",
        skip_serializing_if = "<[_]>::is_empty"
    )]
    #[validate]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}
