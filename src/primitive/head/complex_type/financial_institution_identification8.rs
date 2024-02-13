use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct FinancialInstitutionIdentification8 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub bicfi: Option<crate::primitive::BICFIIdentifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub clr_sys_mmb_id: Option<super::ClearingSystemMemberIdentification2>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nm: Option<crate::primitive::Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pstl_adr: Option<super::PostalAddress6>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub othr: Option<super::GenericFinancialIdentification1>,
}
