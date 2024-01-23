use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GenericOrganisationIdentification1 {
    #[serde(rename = "Id")]
    #[validate]
    pub id: super::super::simple_type::Max35Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<super::OrganisationIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<super::super::simple_type::Max35Text>,
}