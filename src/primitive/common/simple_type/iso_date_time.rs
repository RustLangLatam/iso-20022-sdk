
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct ISODateTime {
    #[serde(rename = "$text")]
    pub value: String,
}
