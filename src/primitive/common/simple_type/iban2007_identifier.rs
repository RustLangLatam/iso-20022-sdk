::lazy_static::lazy_static! {
    pub(super) static ref IBAN2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct IBAN2007Identifier {
    #[validate(regex = "IBAN2007_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
