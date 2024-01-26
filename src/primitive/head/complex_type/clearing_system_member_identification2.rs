use ::validator::Validate;

use crate::primitive::Max35Text;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ClearingSystemMemberIdentification2 {
    #[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub clr_sys_id: Option<super::ClearingSystemIdentification2Choice>,
    #[serde(rename = "MmbId")]
    #[validate]
    pub mmb_id: Max35Text,
}