use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OtherContact1 {
    #[serde(rename = "ChanlTp")]
    #[validate]
    pub chanl_tp: super::super::simple_type::Max4Text,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<super::super::simple_type::Max128Text>,
}