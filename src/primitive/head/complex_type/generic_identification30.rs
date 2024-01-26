use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GenericIdentification30 {
    #[serde(rename = "Id")]
    #[validate]
    pub id: crate::primitive::Exact4AlphaNumericText,
    #[serde(rename = "Issr")]
    #[validate]
    pub issr: crate::primitive::Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub schme_nm: Option<crate::primitive::Max35Text>,
}