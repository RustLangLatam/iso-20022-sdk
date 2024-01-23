use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FIToFIPaymentStatusReportV12<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "GrpHdr")]
    #[validate]
    pub grp_hdr: super::GroupHeader101,
    #[serde(rename = "OrgnlGrpInfAndSts")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub orgnl_grp_inf_and_sts: Vec<super::OriginalGroupHeader17>,
    #[serde(rename = "TxInfAndSts")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub tx_inf_and_sts: Vec<super::PaymentTransaction130<A>>,
    #[serde(rename = "SplmtryData")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}