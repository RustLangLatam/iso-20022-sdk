use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OtherContact1 {
    #[serde(rename = "ChanlTp")]
    #[validate]
    pub chanl_tp: crate::primitive::Max4Text,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub id: Option<crate::primitive::Max128Text>,
}
