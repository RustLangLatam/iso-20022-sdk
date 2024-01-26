use ::validator::Validate;

use crate::primitive::{Max140Text, Max35Text};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct BranchData2 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub id: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pstl_adr: Option<super::PostalAddress6>,
}