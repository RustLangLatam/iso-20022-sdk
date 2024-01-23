use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct EquivalentAmount2 {
    #[serde(rename = "Amt")]
    #[validate]
    pub amt: super::ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyOfTrf")]
    #[validate]
    pub ccy_of_trf: super::super::simple_type::ActiveOrHistoricCurrencyCode,
}