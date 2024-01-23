use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentReturnV11<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "GrpHdr")]
    #[validate]
    pub grp_hdr: super::GroupHeader99,
    #[serde(rename = "OrgnlGrpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf: Option<super::OriginalGroupHeader18>,
    #[serde(rename = "TxInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub tx_inf: Vec<super::PaymentTransaction133<A>>,
    #[serde(rename = "SplmtryData")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}