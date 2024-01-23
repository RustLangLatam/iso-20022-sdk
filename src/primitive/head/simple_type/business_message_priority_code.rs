
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct BusinessMessagePriorityCode {
        #[serde(rename = "$text")]
    pub value: String,
}
