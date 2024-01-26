use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GenericIdentification30 {
    #[serde(rename = "Id")]
    #[validate]
    pub id: super::super::simple_type::Exact4AlphaNumericText,
    #[serde(rename = "Issr")]
    #[validate]
    pub issr: super::super::simple_type::Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub schme_nm: Option<super::super::simple_type::Max35Text>,
}