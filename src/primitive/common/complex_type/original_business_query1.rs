use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct OriginalBusinessQuery1 {
    #[serde(rename = "MsgId")]
    #[validate]
    pub msg_id: super::super::simple_type::Max35Text,
    #[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub msg_nm_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cre_dt_tm: Option<super::super::simple_type::ISODateTime>,
}
