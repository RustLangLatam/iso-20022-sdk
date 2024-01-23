use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "$value")]
    #[validate]
    pub value: super::super::simple_type::ActiveCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    #[validate]
    pub ccy: super::super::simple_type::ActiveCurrencyCode,
}
