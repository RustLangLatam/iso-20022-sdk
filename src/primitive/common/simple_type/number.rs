#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct Number {
    #[serde(rename = "$text")]
    pub value: ::rust_decimal::Decimal,
}
