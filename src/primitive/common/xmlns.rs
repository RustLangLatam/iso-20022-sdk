use std::ops::Deref;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Xmlns {
    #[serde(rename = "@xmlns")]
    pub value: String,
}

impl Deref for Xmlns {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}