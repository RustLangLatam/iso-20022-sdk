#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ContactDetails2 {
    #[serde(rename = "NmPrfx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<super::super::simple_type::NamePrefix1Code>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<crate::primitive::Max140Text>,
    #[serde(rename = "PhneNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "MobNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "FaxNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "EmailAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<crate::primitive::Max2048Text>,
    #[serde(rename = "Othr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub othr: Option<crate::primitive::Max35Text>,
}