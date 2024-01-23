
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct ISOYear {
    #[validate(range(min = 4, max = 4))]
    #[serde(rename = "$text")]
    pub value: u32,
}
