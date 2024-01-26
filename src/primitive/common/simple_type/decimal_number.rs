#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct DecimalNumber {
    #[serde(rename = "$text")]
    pub value: ::rust_decimal::Decimal,
}
