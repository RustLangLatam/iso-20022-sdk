use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DatePeriod2 {
    #[serde(rename = "FrDt")]
    #[validate]
    pub fr_dt: super::super::simple_type::ISODate,
    #[serde(rename = "ToDt")]
    #[validate]
    pub to_dt: super::super::simple_type::ISODate,
}