#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OrganisationIdentification29 {
    #[serde(rename = "AnyBIC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<crate::primitive::AnyBICDec2014Identifier>,
    #[serde(rename = "LEI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<super::super::simple_type::LEIIdentifier>,
    #[serde(rename = "Othr")]
    #[validate(length(min = 0, ))]
    #[serde(default)]
    pub othr: Vec<super::GenericOrganisationIdentification1>,
}