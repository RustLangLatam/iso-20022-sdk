
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max140Text {
    #[validate(length(min = 1, max = 140))]
    #[serde(rename = "$text")]
    pub value: String,
}
