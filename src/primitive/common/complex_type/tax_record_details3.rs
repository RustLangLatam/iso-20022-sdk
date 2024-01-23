use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxRecordDetails3 {
    #[serde(rename = "Prd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prd: Option<super::TaxPeriod3>,
    #[serde(rename = "Amt")]
    #[validate]
    pub amt: super::ActiveOrHistoricCurrencyAndAmount,
}