#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max10Text {
    #[validate(length(min = 1, max = 10))]
    #[serde(rename = "$text")]
    pub value: String,
}
