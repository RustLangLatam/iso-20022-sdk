
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct ISODate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
