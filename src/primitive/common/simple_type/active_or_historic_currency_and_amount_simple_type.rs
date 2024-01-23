
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
    #[serde(rename = "$text")]
    pub value: ::rust_decimal::Decimal,
}
