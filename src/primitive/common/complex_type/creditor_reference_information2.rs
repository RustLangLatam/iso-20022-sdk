#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditorReferenceInformation2 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::CreditorReferenceType2>,
    #[serde(rename = "Ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<super::super::simple_type::Max35Text>,
}