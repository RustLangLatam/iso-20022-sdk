use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FrequencyPeriod1 {
    #[serde(rename = "Tp")]
    pub tp: super::super::simple_type::Frequency6Code,
    #[serde(rename = "CntPerPrd")]
    #[validate]
    pub cnt_per_prd: super::super::simple_type::DecimalNumber,
}
