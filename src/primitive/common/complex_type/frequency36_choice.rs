use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Frequency36Choice {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::super::simple_type::Frequency6Code>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prd: Option<super::FrequencyPeriod1>,
    #[serde(rename = "PtInTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pt_in_tm: Option<super::FrequencyAndMoment1>,
}