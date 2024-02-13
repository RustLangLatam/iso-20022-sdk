use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FinancialInstitutionIdentification18 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub bicfi: Option<crate::primitive::BICFIDec2014Identifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub clr_sys_mmb_id: Option<super::ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub lei: Option<crate::primitive::LEIIdentifier>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nm: Option<crate::primitive::Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pstl_adr: Option<super::PostalAddress24>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub othr: Option<super::GenericFinancialIdentification1>,
}
