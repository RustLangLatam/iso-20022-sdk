#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct ExternalLocalInstrument1Code {
    #[validate(length(min = 1, max = 35))]
    #[serde(rename = "$text")]
    pub value: String,
}
