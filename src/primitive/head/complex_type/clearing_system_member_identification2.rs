use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ClearingSystemMemberIdentification2 {
    #[serde(rename = "ClrSysId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_id: Option<super::ClearingSystemIdentification2Choice>,
    #[serde(rename = "MmbId")]
    #[validate]
    pub mmb_id: crate::primitive::Max35Text,
}