
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max350Text {
    #[validate(length(min = 1, max = 350))]
    #[serde(rename = "$text")]
    pub value: String,
}
