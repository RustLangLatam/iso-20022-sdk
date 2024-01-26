use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentReturnV11<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "GrpHdr")]
    #[validate]
    pub grp_hdr: super::GroupHeader99,
    #[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_grp_inf: Option<super::OriginalGroupHeader18>,
    #[serde(default, rename = "TxInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub tx_inf: Vec<super::PaymentTransaction133<A>>,
    #[serde(default, rename = "SplmtryData", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}