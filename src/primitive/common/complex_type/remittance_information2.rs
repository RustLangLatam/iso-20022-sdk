#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RemittanceInformation2 {
    #[serde(rename = "Ustrd")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub ustrd: Vec<super::super::simple_type::Max140Text>,
}