#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct UnicodeChartsCode {
    #[serde(rename = "$text")]
    pub value: String,
}
