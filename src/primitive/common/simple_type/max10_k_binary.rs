#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Max10KBinary {
    #[validate(length(min = 1, max = 10240))]
    #[serde(rename = "$text")]
    pub value: String,
}
