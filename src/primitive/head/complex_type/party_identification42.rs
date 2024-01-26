use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PartyIdentification42 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nm: Option<crate::primitive::Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pstl_adr: Option<super::PostalAddress6>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub id: Option<super::Party10Choice>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctry_of_res: Option<crate::primitive::CountryCode>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctct_dtls: Option<super::ContactDetails2>,
}