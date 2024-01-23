
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max105Text {
    #[validate(length(min = 1, max = 105))]
    #[serde(rename = "$text")]
    pub value: String,
}
