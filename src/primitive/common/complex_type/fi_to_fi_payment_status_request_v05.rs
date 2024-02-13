use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FIToFIPaymentStatusRequestV05<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "GrpHdr")]
    #[validate]
    pub grp_hdr: super::GroupHeader91,
    #[serde(
        default,
        rename = "OrgnlGrpInf",
        skip_serializing_if = "<[_]>::is_empty"
    )]
    #[validate]
    pub orgnl_grp_inf: Vec<super::OriginalGroupInformation27>,
    #[serde(default, rename = "TxInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub tx_inf: Vec<super::PaymentTransaction131<A>>,
    #[serde(
        default,
        rename = "SplmtryData",
        skip_serializing_if = "<[_]>::is_empty"
    )]
    #[validate]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}
