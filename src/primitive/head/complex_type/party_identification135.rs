#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PartyIdentification135 {
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<crate::primitive::Max140Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<super::PostalAddress24>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::primitive::Party38Choice>,
    #[serde(rename = "CtryOfRes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<crate::primitive::CountryCode>,
    #[serde(rename = "CtctDtls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<super::Contact4>,
}