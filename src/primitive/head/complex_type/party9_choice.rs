use crate::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Party9Choice {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub org_id: Option<super::PartyIdentification42>,
    #[serde(rename = "FIId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub fi_id: Option<super::BranchAndFinancialInstitutionIdentification5>,
}
