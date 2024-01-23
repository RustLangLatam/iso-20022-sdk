#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxAmount3 {
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<super::super::simple_type::PercentageRate>,
    #[serde(rename = "TaxblBaseAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxbl_base_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dtls")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub dtls: Vec<super::TaxRecordDetails3>,
}