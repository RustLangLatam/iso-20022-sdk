use crate::primitive::{Max140Text, Max35Text};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct BranchData2 {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<super::PostalAddress6>,
}