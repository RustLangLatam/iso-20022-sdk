#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CashAccount40 {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<super::AccountIdentification4Choice>,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::CashAccountType2Choice>,
    #[serde(rename = "Ccy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<super::super::simple_type::ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<super::super::simple_type::Max70Text>,
    #[serde(rename = "Prxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prxy: Option<super::ProxyAccountIdentification1>,
}