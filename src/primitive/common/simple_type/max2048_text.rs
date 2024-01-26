#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max2048Text {
    #[validate(length(min = 1, max = 2048))]
    #[serde(rename = "$text")]
    pub value: String,
}
