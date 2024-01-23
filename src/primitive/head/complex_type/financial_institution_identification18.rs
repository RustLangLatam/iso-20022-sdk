#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FinancialInstitutionIdentification18 {
    #[serde(rename = "BICFI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<crate::primitive::BICFIDec2014Identifier>,
    #[serde(rename = "ClrSysMmbId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<super::ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<super::super::simple_type::LEIIdentifier>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<crate::primitive::Max140Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<super::PostalAddress24>,
    #[serde(rename = "Othr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub othr: Option<super::GenericFinancialIdentification1>,
}