use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxPeriod3 {
    #[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub yr: Option<super::super::simple_type::ISOYear>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::super::simple_type::TaxRecordPeriod1Code>,
    #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub fr_to_dt: Option<super::DatePeriod2>,
}
