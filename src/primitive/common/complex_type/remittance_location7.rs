use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RemittanceLocation7 {
    #[serde(rename = "RmtId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rmt_id: Option<super::super::simple_type::Max35Text>,
    #[serde(default, rename = "RmtLctnDtls", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub rmt_lctn_dtls: Vec<super::RemittanceLocationData1>,
}