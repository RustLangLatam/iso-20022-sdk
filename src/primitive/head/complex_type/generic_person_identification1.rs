use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GenericPersonIdentification1 {
    #[serde(rename = "Id")]
    #[validate]
    pub id: crate::primitive::Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub schme_nm: Option<crate::primitive::PersonIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub issr: Option<crate::primitive::Max35Text>,
}
