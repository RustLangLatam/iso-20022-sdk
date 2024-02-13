use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OrganisationIdentification29 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub any_bic: Option<super::super::simple_type::AnyBICDec2014Identifier>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub lei: Option<super::super::simple_type::LEIIdentifier>,
    #[serde(default, rename = "Othr", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub othr: Vec<super::GenericOrganisationIdentification1>,
}
