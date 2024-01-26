use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ContactDetails2 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<crate::primitive::NamePrefix1Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nm: Option<crate::primitive::Max140Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub phne_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub mob_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub fax_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub email_adr: Option<crate::primitive::Max2048Text>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub othr: Option<crate::primitive::Max35Text>,
}