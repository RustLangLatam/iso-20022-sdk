use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "$value")]
    #[validate]
    pub value: super::super::simple_type::ActiveOrHistoricCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    #[validate]
    pub ccy: super::super::simple_type::ActiveOrHistoricCurrencyCode,
}
