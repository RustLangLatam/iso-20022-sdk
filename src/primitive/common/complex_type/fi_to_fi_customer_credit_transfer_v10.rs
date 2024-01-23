use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FIToFICustomerCreditTransferV10<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "GrpHdr")]
    #[validate]
    pub grp_hdr: super::GroupHeader96,
    #[serde(rename = "CdtTrfTxInf")]
    #[validate(length(min = 1,))]
    pub cdt_trf_tx_inf: Vec<super::CreditTransferTransaction50<A>>,
    #[serde(default, rename = "SplmtryData", skip_serializing_if = "<[_]>::is_empty")]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}