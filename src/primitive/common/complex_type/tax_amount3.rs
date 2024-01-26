use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxAmount3 {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rate: Option<super::super::simple_type::PercentageRate>,
    #[serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub taxbl_base_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ttl_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(default, rename = "Dtls", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub dtls: Vec<super::TaxRecordDetails3>,
}