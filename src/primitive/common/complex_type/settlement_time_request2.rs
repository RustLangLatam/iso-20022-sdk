use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct SettlementTimeRequest2 {
    #[serde(rename = "CLSTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cls_tm: Option<super::super::simple_type::ISOTime>,
    #[serde(rename = "TillTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub till_tm: Option<super::super::simple_type::ISOTime>,
    #[serde(rename = "FrTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub fr_tm: Option<super::super::simple_type::ISOTime>,
    #[serde(rename = "RjctTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rjct_tm: Option<super::super::simple_type::ISOTime>,
}