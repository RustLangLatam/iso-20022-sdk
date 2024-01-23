::lazy_static::lazy_static! {
    pub(super) static ref UUI_DV4_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}"#).unwrap();
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct UUIDv4Identifier {
    #[validate(regex = "UUI_DV4_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
