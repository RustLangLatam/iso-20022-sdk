use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FinancialInstitutionDirectDebitV05<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "GrpHdr")]
    #[validate]
    pub grp_hdr: super::GroupHeader92,
    #[serde(rename = "CdtInstr")]
    #[validate(length(min = 1,))]
    pub cdt_instr: Vec<super::CreditTransferTransaction53<A>>,
    #[serde(default, rename = "SplmtryData", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub splmtry_data: Vec<super::SupplementaryData1<A>>,
}