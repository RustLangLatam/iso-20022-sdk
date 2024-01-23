#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RemittanceInformation21 {
    #[serde(rename = "Ustrd")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub ustrd: Vec<super::super::simple_type::Max140Text>,
    #[serde(rename = "Strd")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub strd: Vec<super::StructuredRemittanceInformation17>,
}