
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct ExternalClearingSystemIdentification1Code {
    #[validate(length(min = 1, max = 5))]
    #[serde(rename = "$text")]
    pub value: String,
}
