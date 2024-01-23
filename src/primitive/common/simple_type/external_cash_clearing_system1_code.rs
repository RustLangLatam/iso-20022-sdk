
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct ExternalCashClearingSystem1Code {
    #[validate(length(min = 1, max = 3))]
    #[serde(rename = "$text")]
    pub value: String,
}
