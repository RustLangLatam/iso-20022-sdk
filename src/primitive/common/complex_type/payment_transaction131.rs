#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentTransaction131<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "StsReqId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sts_req_id: Option<super::super::simple_type::Max35Text>,
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
    #[serde(rename = "AccptncDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accptnc_dt_tm: Option<super::super::simple_type::ISODateTime>,
    #[serde(rename = "ClrSysRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlTxRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref: Option<super::OriginalTransactionReference35>,
    #[serde(rename = "SplmtryData")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}