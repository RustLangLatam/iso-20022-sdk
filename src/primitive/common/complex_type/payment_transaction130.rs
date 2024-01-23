#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentTransaction130<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "StsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sts_id: Option<super::super::simple_type::Max35Text>,
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
    #[serde(rename = "TxSts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_sts: Option<super::super::simple_type::ExternalPaymentTransactionStatus1Code>,
    #[serde(rename = "StsRsnInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub sts_rsn_inf: Vec<super::StatusReasonInformation12>,
    #[serde(rename = "ChrgsInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub chrgs_inf: Vec<super::Charges7>,
    #[serde(rename = "AccptncDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accptnc_dt_tm: Option<super::super::simple_type::ISODateTime>,
    #[serde(rename = "FctvIntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fctv_intr_bk_sttlm_dt: Option<super::DateAndDateTime2Choice>,
    #[serde(rename = "AcctSvcrRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref: Option<super::super::simple_type::Max35Text>,
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