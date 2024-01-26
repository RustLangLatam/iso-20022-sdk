use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RemittanceInformation21 {
    #[serde(default, rename = "Ustrd", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub ustrd: Vec<super::super::simple_type::Max140Text>,
    #[serde(default, rename = "Strd", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub strd: Vec<super::StructuredRemittanceInformation17>,
}