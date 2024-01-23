
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max1025Text {
    #[validate(length(min = 1, max = 1025))]
    #[serde(rename = "$text")]
    pub value: String,
}
