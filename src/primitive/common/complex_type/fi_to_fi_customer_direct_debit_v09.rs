use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FIToFICustomerDirectDebitV09<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "GrpHdr")]
    #[validate]
    pub grp_hdr: super::GroupHeader98,
    #[serde(rename = "DrctDbtTxInf")]
    #[validate(length(min = 1,))]
    pub drct_dbt_tx_inf: Vec<super::DirectDebitTransactionInformation29<A>>,
    #[serde(
        default,
        rename = "SplmtryData",
        skip_serializing_if = "<[_]>::is_empty"
    )]
    #[validate]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}
