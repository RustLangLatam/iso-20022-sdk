#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OrganisationIdentification7 {
    #[serde(rename = "AnyBIC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<super::super::simple_type::AnyBICIdentifier>,
    #[serde(rename = "Othr")]
    #[validate(length(min = 0, ))]
    #[serde(default)]
    pub othr: Vec<super::GenericOrganisationIdentification1>,
}