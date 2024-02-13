use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentTransaction130<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "StsId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sts_id: Option<super::super::simple_type::Max35Text>,
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
    #[serde(rename = "TxSts", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tx_sts: Option<super::super::simple_type::ExternalPaymentTransactionStatus1Code>,
    #[serde(default, rename = "StsRsnInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub sts_rsn_inf: Vec<super::StatusReasonInformation12>,
    #[serde(default, rename = "ChrgsInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub chrgs_inf: Vec<super::Charges7>,
    #[serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub accptnc_dt_tm: Option<super::super::simple_type::ISODateTime>,
    #[serde(rename = "FctvIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub fctv_intr_bk_sttlm_dt: Option<super::DateAndDateTime2Choice>,
    #[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub acct_svcr_ref: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub clr_sys_ref: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instg_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instd_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_tx_ref: Option<super::OriginalTransactionReference35>,
    #[serde(
        default,
        rename = "SplmtryData",
        skip_serializing_if = "<[_]>::is_empty"
    )]
    #[validate]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}
