#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct SettlementTimeRequest2 {
    #[serde(rename = "CLSTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cls_tm: Option<super::super::simple_type::ISOTime>,
    #[serde(rename = "TillTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub till_tm: Option<super::super::simple_type::ISOTime>,
    #[serde(rename = "FrTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_tm: Option<super::super::simple_type::ISOTime>,
    #[serde(rename = "RjctTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rjct_tm: Option<super::super::simple_type::ISOTime>,
}