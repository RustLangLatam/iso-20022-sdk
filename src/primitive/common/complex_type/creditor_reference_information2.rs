use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditorReferenceInformation2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tp: Option<super::CreditorReferenceType2>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub r#ref: Option<super::super::simple_type::Max35Text>,
}
