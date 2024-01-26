use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FIToFIPaymentStatusReportV12<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "GrpHdr")]
    #[validate]
    pub grp_hdr: super::GroupHeader101,
    #[serde(default, rename = "OrgnlGrpInfAndSts", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub orgnl_grp_inf_and_sts: Vec<super::OriginalGroupHeader17>,
    #[serde(default, rename = "TxInfAndSts", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub tx_inf_and_sts: Vec<super::PaymentTransaction130<A>>,
    #[serde(default, rename = "SplmtryData", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}