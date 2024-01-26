#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max16Text {
    #[validate(length(min = 1, max = 16))]
    #[serde(rename = "$text")]
    pub value: String,
}
