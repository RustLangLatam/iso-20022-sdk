use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GenericPersonIdentification1 {
    #[serde(rename = "Id")]
    #[validate]
    pub id: crate::primitive::Max35Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<crate::primitive::PersonIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<crate::primitive::Max35Text>,
}