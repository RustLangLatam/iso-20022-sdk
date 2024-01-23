
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max128Text {
    #[validate(length(min = 1, max = 128))]
    #[serde(rename = "$text")]
    pub value: String,
}
