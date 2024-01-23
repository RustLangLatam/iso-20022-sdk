use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct BranchAndFinancialInstitutionIdentification5 {
    #[serde(rename = "FinInstnId")]
    #[validate]
    pub fin_instn_id: super::FinancialInstitutionIdentification8,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub brnch_id: Option<super::BranchData2>,
}