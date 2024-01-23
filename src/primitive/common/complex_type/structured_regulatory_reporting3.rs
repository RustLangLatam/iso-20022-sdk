#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct StructuredRegulatoryReporting3 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Dt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "Ctry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry: Option<super::super::simple_type::CountryCode>,
    #[serde(rename = "Cd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cd: Option<super::super::simple_type::Max10Text>,
    #[serde(rename = "Amt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Inf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub inf: Vec<super::super::simple_type::Max35Text>,
}