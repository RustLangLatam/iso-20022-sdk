#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RemittanceLocation7 {
    #[serde(rename = "RmtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "RmtLctnDtls")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub rmt_lctn_dtls: Vec<super::RemittanceLocationData1>,
}