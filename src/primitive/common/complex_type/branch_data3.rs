#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct BranchData3 {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "LEI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<super::super::simple_type::LEIIdentifier>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<super::super::simple_type::Max140Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<super::PostalAddress24>,
}