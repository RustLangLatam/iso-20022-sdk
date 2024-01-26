use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CashAccount40 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub id: Option<super::AccountIdentification4Choice>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tp: Option<super::CashAccountType2Choice>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ccy: Option<super::super::simple_type::ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nm: Option<super::super::simple_type::Max70Text>,
    #[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prxy: Option<super::ProxyAccountIdentification1>,
}