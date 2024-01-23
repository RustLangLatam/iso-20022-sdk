use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct BranchAndFinancialInstitutionIdentification6 {
    #[serde(rename = "FinInstnId")]
    #[validate]
    pub fin_instn_id: super::FinancialInstitutionIdentification18,
    #[serde(rename = "BrnchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<super::BranchData3>,
}