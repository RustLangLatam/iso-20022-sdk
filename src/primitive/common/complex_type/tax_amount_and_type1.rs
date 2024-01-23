use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxAmountAndType1 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::TaxAmountType1Choice>,
    #[serde(rename = "Amt")]
    #[validate]
    pub amt: super::ActiveOrHistoricCurrencyAndAmount,
}