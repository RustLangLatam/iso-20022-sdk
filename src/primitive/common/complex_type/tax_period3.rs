#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxPeriod3 {
    #[serde(rename = "Yr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yr: Option<super::super::simple_type::ISOYear>,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::super::simple_type::TaxRecordPeriod1Code>,
    #[serde(rename = "FrToDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<super::DatePeriod2>,
}