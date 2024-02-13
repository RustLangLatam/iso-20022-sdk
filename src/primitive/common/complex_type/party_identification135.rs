use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PartyIdentification135 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nm: Option<super::super::simple_type::Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pstl_adr: Option<super::PostalAddress24>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub id: Option<super::Party38Choice>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctry_of_res: Option<super::super::simple_type::CountryCode>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctct_dtls: Option<super::Contact4>,
}
