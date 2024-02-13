use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentTransaction133<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "RtrId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rtr_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_grp_inf: Option<super::OriginalGroupInformation29>,
    #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_instr_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_end_to_end_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_tx_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_uetr: Option<super::super::simple_type::UUIDv4Identifier>,
    #[serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_clr_sys_ref: Option<super::super::simple_type::Max35Text>,
    #[serde(
        rename = "OrgnlIntrBkSttlmAmt",
        skip_serializing_if = "Option::is_none"
    )]
    #[validate]
    pub orgnl_intr_bk_sttlm_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_intr_bk_sttlm_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pmt_tp_inf: Option<super::PaymentTypeInformation28>,
    #[serde(rename = "RtrdIntrBkSttlmAmt")]
    #[validate]
    pub rtrd_intr_bk_sttlm_amt: super::ActiveCurrencyAndAmount,
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
    #[serde(rename = "RtrdInstdAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rtrd_instd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub xchg_rate: Option<super::super::simple_type::BaseOneRate>,
    #[serde(rename = "CompstnAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub compstn_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none")]
    pub chrg_br: Option<super::super::simple_type::ChargeBearerType1Code>,
    #[serde(default, rename = "ChrgsInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub chrgs_inf: Vec<super::Charges7>,
    #[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub clr_sys_ref: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instg_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instd_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "RtrChain", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rtr_chain: Option<super::TransactionParties10>,
    #[serde(default, rename = "RtrRsnInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub rtr_rsn_inf: Vec<super::PaymentReturnReason6>,
    #[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_tx_ref: Option<super::OriginalTransactionReference36>,
    #[serde(
        default,
        rename = "SplmtryData",
        skip_serializing_if = "<[_]>::is_empty"
    )]
    #[validate]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}
