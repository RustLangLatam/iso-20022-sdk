use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FIToFIPaymentStatusRequestV05<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "GrpHdr")]
    #[validate]
    pub grp_hdr: super::GroupHeader91,
    #[serde(rename = "OrgnlGrpInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub orgnl_grp_inf: Vec<super::OriginalGroupInformation27>,
    #[serde(rename = "TxInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub tx_inf: Vec<super::PaymentTransaction131<A>>,
    #[serde(rename = "SplmtryData")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}