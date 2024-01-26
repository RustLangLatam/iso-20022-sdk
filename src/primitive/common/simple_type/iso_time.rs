#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct ISOTime {
    #[serde(rename = "$text")]
    pub value: ::chrono::naive::NaiveTime,
}
