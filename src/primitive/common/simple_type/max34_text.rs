#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max34Text {
    #[validate(length(min = 1, max = 34))]
    #[serde(rename = "$text")]
    pub value: String,
}
