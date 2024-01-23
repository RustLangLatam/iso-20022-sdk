use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GenericAccountIdentification1 {
    #[serde(rename = "Id")]
    #[validate]
    pub id: super::super::simple_type::Max34Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<super::AccountSchemeName1Choice>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<super::super::simple_type::Max35Text>,
}