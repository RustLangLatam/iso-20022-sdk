use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FinancialIdentificationSchemeName1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<crate::primitive::ExternalFinancialInstitutionIdentification1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prtry: Option<crate::primitive::Max35Text>,
}