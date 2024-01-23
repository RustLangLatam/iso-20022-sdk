::lazy_static::lazy_static! {
    pub(super) static ref BICFI_DEC2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct BICFIDec2014Identifier {
    #[validate(regex = "BICFI_DEC2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}