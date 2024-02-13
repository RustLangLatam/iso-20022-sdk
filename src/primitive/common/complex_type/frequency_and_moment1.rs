use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FrequencyAndMoment1 {
    #[serde(rename = "Tp")]
    pub tp: super::super::simple_type::Frequency6Code,
    #[serde(rename = "PtInTm")]
    #[validate]
    pub pt_in_tm: super::super::simple_type::Exact2NumericText,
}
