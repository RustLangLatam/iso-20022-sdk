
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max70Text {
    #[validate(length(min = 1, max = 70))]
    #[serde(rename = "$text")]
    pub value: String,
}
