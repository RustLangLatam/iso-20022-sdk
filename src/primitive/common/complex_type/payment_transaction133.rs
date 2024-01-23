use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentTransaction133<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "RtrId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtr_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlGrpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf: Option<super::OriginalGroupInformation29>,
    #[serde(rename = "OrgnlInstrId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlEndToEndId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_end_to_end_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlTxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlUETR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_uetr: Option<super::super::simple_type::UUIDv4Identifier>,
    #[serde(rename = "OrgnlClrSysRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_clr_sys_ref: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlIntrBkSttlmAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_intr_bk_sttlm_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "OrgnlIntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation28>,
    #[serde(rename = "RtrdIntrBkSttlmAmt")]
    #[validate]
    pub rtrd_intr_bk_sttlm_amt: super::ActiveCurrencyAndAmount,
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
    #[serde(rename = "RtrdInstdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtrd_instd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "XchgRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<super::super::simple_type::BaseOneRate>,
    #[serde(rename = "CompstnAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compstn_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "ChrgBr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrg_br: Option<super::super::simple_type::ChargeBearerType1Code>,
    #[serde(rename = "ChrgsInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub chrgs_inf: Vec<super::Charges7>,
    #[serde(rename = "ClrSysRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "RtrChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtr_chain: Option<super::TransactionParties10>,
    #[serde(rename = "RtrRsnInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub rtr_rsn_inf: Vec<super::PaymentReturnReason6>,
    #[serde(rename = "OrgnlTxRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref: Option<super::OriginalTransactionReference36>,
    #[serde(rename = "SplmtryData")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}