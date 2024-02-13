use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct StructuredRegulatoryReporting3 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tp: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctry: Option<super::super::simple_type::CountryCode>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cd: Option<super::super::simple_type::Max10Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(default, rename = "Inf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub inf: Vec<super::super::simple_type::Max35Text>,
}
