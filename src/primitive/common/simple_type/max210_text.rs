#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max210Text {
    #[validate(length(min = 1, max = 210))]
    #[serde(rename = "$text")]
    pub value: String,
}
